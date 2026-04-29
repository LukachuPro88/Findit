# Findit

Findit is a high-performance **local search engine** and CLI tool written in C++. It is designed to index local directories and provide near-instant text retrieval through an inverted index system.

---

## Features
*   **File Discovery**: Recursively scan directories to identify searchable files using `std::filesystem`.
*   **Full-Text Search**: Search for specific words across multiple files simultaneously.
*   **Folder Targeting**: Limit the search scope to specific directory trees.
*   **Inverted Indexing**: Optimized data structures ensure $O(1)$ lookup speeds regardless of the number of files.

---

## System Architecture

Findit follows a modular systems engineering pipeline:

1.  **Ingestor**: Discovers files and handles stream-based reading to manage memory efficiency.
2.  **Indexer**: Tokenizes raw text, normalizes strings (lowercasing/punctuation stripping), and maps words to their file locations.
3.  **Query Engine**: Processes user input and intersects index data to return relevant results.

---

## Development Requirements
*   **Language**: C++17 or later
*   **Compiler**: GCC / G++
*   **Build System**: CMake 3.10+

---
