#!/bin/sh

set -e

BINARY="findit"
INSTALL_DIR="/usr/local/bin"
REPO="LukachuPro88/Findit"
VERSION="0.1.0"

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" != "Linux" ]; then
    echo "Unsupported OS: $OS"
    echo "Please install via cargo: cargo install findit-rs"
    exit 1
fi

if [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-unknown-linux-gnu"
elif [ "$ARCH" = "aarch64" ]; then
    TARGET="aarch64-unknown-linux-gnu"
else
    echo "Unsupported architecture: $ARCH"
    echo "Please install via cargo: cargo install findit-rs"
    exit 1
fi

URL="https://github.com/$REPO/releases/download/v$VERSION/$BINARY-$TARGET"

echo "Downloading findit v$VERSION..."
curl -sSL "$URL" -o "/tmp/$BINARY"
chmod +x "/tmp/$BINARY"

echo "Installing to $INSTALL_DIR (may require sudo)..."
if [ -w "$INSTALL_DIR" ]; then
    mv "/tmp/$BINARY" "$INSTALL_DIR/$BINARY"
else
    sudo mv "/tmp/$BINARY" "$INSTALL_DIR/$BINARY"
fi

echo "findit installed successfully!"
echo "Run 'findit' to get started."
