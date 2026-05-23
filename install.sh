#!/bin/sh
set -e

BINARY="findit"
INSTALL_DIR="/usr/local/bin"
REPO="LukachuPro88/Findit"

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" != "Linux" ]; then
    echo "Unsupported OS: $OS"
    echo "This installer script targets Linux environments."
    exit 1
fi

if [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-unknown-linux-gnu"
elif [ "$ARCH" = "aarch64" ]; then
    TARGET="aarch64-unknown-linux-gnu"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

echo "Fetching latest release version..."
VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "Failed to fetch the latest version tag from GitHub."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$VERSION/findit-rs-$TARGET"

echo "Downloading findit $VERSION for $TARGET..."
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
