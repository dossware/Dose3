$GITHUB_REPO = "donnie3237/DosE3" 
# ----------------------------------------------------
$EXE_NAME = "dose.exe"
$OLD_EXE_NAME = "dose3.exe" 
# --------------------------------------------------------------------------
$AppName = $EXE_NAME.Split('.')[0]
$InstallPath = "$env:USERPROFILE\.$AppName"
$BinPath = Join-Path $InstallPath "bin"

function Write-Host-Color($Message, $Color) {
    Write-Host $Message -ForegroundColor $Color
}

Write-Host-Color "Starting installation of $AppName..." -Color Cyan
Write-Host "Installation path: $InstallPath"

Write-Host "Fetching latest release from GitHub..."
try {
    $ApiUrl = "https://api.github.com/repos/$GITHUB_REPO/releases/latest"
    $release = Invoke-RestMethod -Uri $ApiUrl
    $asset = $release.assets | Where-Object { $_.name -like '*dose3-windows-x64.zip' } | Select-Object -First 1
    if (-not $asset) { throw "Could not find a suitable 'dose3-windows-x64.zip' asset in the latest release." }
    $DownloadUrl = $asset.browser_download_url
    Write-Host-Color "Found version: $($release.tag_name)" -Color Green
} catch {
    Write-Host-Color "Error: Failed to fetch release information." -Color Red; exit 1
}
$TempZipFile = Join-Path $env:TEMP "$($asset.name)"
Write-Host "Downloading $($asset.name)..."
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZipFile -UseBasicParsing
    Write-Host-Color "Download complete." -Color Green
} catch {
    Write-Host-Color "Error: Download failed." -Color Red; exit 1
}
if (-not (Test-Path -Path $BinPath)) { New-Item -ItemType Directory -Force -Path $BinPath | Out-Null }
try {
    Expand-Archive -Path $TempZipFile -DestinationPath $BinPath -Force
    Write-Host-Color "Successfully extracted to $BinPath" -Color Green
} catch {
    Write-Host-Color "Error: Failed to extract archive." -Color Red; exit 1
} finally {
    Remove-Item $TempZipFile -ErrorAction SilentlyContinue
}

try {
    $OldFilePath = Join-Path $BinPath $OLD_EXE_NAME
    $NewFilePath = Join-Path $BinPath $EXE_NAME
    if (Test-Path $OldFilePath) {
        Write-Host "Renaming $OLD_EXE_NAME to $EXE_NAME..."
        # Remove existing dose.exe if it exists
        if (Test-Path $NewFilePath) {
            Remove-Item $NewFilePath -Force
        }
        Rename-Item -Path $OldFilePath -NewName $EXE_NAME
        Write-Host-Color "Rename successful." -Color Green
    }
} catch {
    Write-Host-Color "Error: Failed to rename executable." -Color Red
}

Write-Host "Configuring PATH environment variable..."
try {
    $CurrentUserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    if ($CurrentUserPath -notlike "*$BinPath*") {
        $NewPath = ($CurrentUserPath, $BinPath) -join ';'
        [System.Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        Write-Host-Color "`"$BinPath`" has been added to your user PATH." -Color Green
        Write-Host-Color "Please restart your terminal session for the changes to take effect." -Color Yellow
    } else {
        Write-Host-Color "`"$BinPath`" is already in your PATH." -Color White
    }
} catch {
    Write-Host-Color "Error: Failed to update PATH. You may need to add `"$BinPath`" manually." -Color Red
}

Write-Host-Color "Installation successful! 🎉" -Color Magenta
Write-Host "To get started, open a new terminal and run: `"$AppName --help"`"