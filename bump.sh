#!/bin/sh
NEW_VERSION=$1
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
