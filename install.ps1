$ErrorActionPreference = "Stop"

$BINARY = "findit"
$REPO = "LukachuPro88/Findit"
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\findit"
$TARGET = "x86_64-pc-windows-msvc"

Write-Host "Fetching latest release version..."
$VERSION = (Invoke-RestMethod "https://api.github.com/repos/$REPO/releases/latest").tag_name

if (-not $VERSION) {
    Write-Error "Failed to fetch the latest version tag from GitHub."
    exit 1
}

$URL = "https://github.com/$REPO/releases/download/$VERSION/findit-rs-$TARGET.exe"
$TMP = "$env:TEMP\findit.exe"

Write-Host "Downloading findit $VERSION..."
Invoke-WebRequest -Uri $URL -OutFile $TMP

Write-Host "Installing to $INSTALL_DIR..."
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null
Move-Item -Force $TMP "$INSTALL_DIR\$BINARY.exe"

# Add to PATH if not already there
$PATH = [Environment]::GetEnvironmentVariable("Path", "User")
if ($PATH -notlike "*$INSTALL_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$PATH;$INSTALL_DIR", "User")
    Write-Host "Added $INSTALL_DIR to PATH."
}

Write-Host "findit installed successfully!"
Write-Host "Restart your terminal then run 'findit' to get started."
