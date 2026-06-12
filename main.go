package main

/*
#cgo LDFLAGS: -L${SRCDIR}/target/debug -L${SRCDIR}/target/release -lfindit
#include <stdlib.h>

// Link the Go runtime to the compiled C-compatible Rust symbol
void findit_crawl(const char* c_start_path, const char* c_target, const char* mode);
*/
import "C"

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"unsafe"
)

// callRustEngine marshals Go strings into native heap C-strings,
// executes across the FFI boundary, and guarantees memory deallocation.
func callRustEngine(startPath, target, mode string) {
	cStartPath := C.CString(startPath)
	cTarget := C.CString(target)
	cMode := C.CString(mode)

	// Defer free operations to avoid leaking memory on the C heap
	defer C.free(unsafe.Pointer(cStartPath))
	defer C.free(unsafe.Pointer(cTarget))
	defer C.free(unsafe.Pointer(cMode))

	C.findit_crawl(cStartPath, cTarget, cMode)
}

func main() {
	// 1. Establish CLI flag definitions matching your interface rules
	dirPtr := flag.String("dir", "", "Search for directories matching <name>")
	filePtr := flag.String("file", "", "Search for files matching <name>")
	wordPtr := flag.String("word", "", "Search for a specific word inside files")

	flag.Parse()

	var mode, startPath, target string

	// 2. Map options to set operational state variables
	if *dirPtr != "" {
		mode = "dir"
		startPath = *dirPtr
	} else if *filePtr != "" {
		mode = "file"
		startPath = *filePtr
	} else if *wordPtr != "" {
		mode = "word"
		startPath = *wordPtr
	} else {
		printUsage()
		os.Exit(1)
	}

	// Capture the trailing target pattern positional argument
	args := flag.Args()
	if len(args) < 1 {
		fmt.Println("[ERROR] Missing search target.")
		printUsage()
		os.Exit(1)
	}
	target = args[0]

	// 3. Read the initial top-level folder entries to divide the work
	entries, err := os.ReadDir(startPath)
	if err != nil {
		// Fallback: If directory cannot be sub-scanned, process sequentially on the root
		callRustEngine(startPath, target, mode)
		return
	}

	var wg sync.WaitGroup

	// 4. Concurrently process child folders across isolated background threads
	for _, entry := range entries {
		subPath := filepath.Join(startPath, entry.Name())

		// Guardrail: Skip Linux virtual filesystems to eliminate deadlocks or permission noise
		if entry.Name() == "proc" || entry.Name() == "sys" || entry.Name() == "dev" {
			continue
		}

		wg.Add(1)
		go func(p string) {
			defer wg.Done()
			callRustEngine(p, target, mode)
		}(subPath)
	}

	// 5. Block main execution block until all worker threads return
	wg.Wait()
}

func printUsage() {
	fmt.Println("findit-rs VERSION 2.2.0")
	fmt.Println("Usage:")
	fmt.Println("  findit --dir    <start_path> <name>   [--verbose]")
	fmt.Println("  findit --file   <start_path> <name>   [--verbose]")
	fmt.Println("  findit --word   <start_path> <word>   [--verbose]")
	fmt.Println("  findit --ignore <file_path>")
}
