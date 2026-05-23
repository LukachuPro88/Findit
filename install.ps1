$ErrorActionPreference = "Stop"

$Repo = "LukachuPro88/Findit"
$BinaryName = "findit.exe"
$InstallDir = "C:\Windows\System32"

Write-Host "Fetching latest release version from GitHub..." -ForegroundColor Cyan
$ReleaseApi = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
$Tag = $ReleaseApi.tag_name

# Filter the assets to find the windows executable
$Asset = $ReleaseApi.assets | Where-Object { $_.name -like "*windows*" -or $_.name -like "*.exe" } | Select-Object -First 1

if (-not $Asset) {
    Write-Error "No Windows binary (.exe) asset found for release $Tag on GitHub!"
}

$DownloadUrl = $Asset.browser_download_url
$TempPath = Join-Path $env:TEMP $BinaryName

Write-Host "Downloading $BinaryName ($Tag)..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempPath

Write-Host "Installing to $InstallDir (Requires Administrator permissions)..." -ForegroundColor Cyan
try {
    Move-Item -Path $TempPath -Destination (Join-Path $InstallDir $BinaryName) -Force
    Write-Host "findit installed successfully! Run 'findit --g' to get started." -ForegroundColor Green
} catch {
    Write-Host "Installation failed. Please re-run this PowerShell window as an Administrator." -ForegroundColor Red
}
