#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$Repo = "SourisCG/SourisDW"
$Binary = "souris-dw"
$Version = if ($env:VERSION) { $env:VERSION } else { "latest" }

function Write-Info {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Green
}

function Write-Error-Exit {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Red
    exit 1
}

function Get-DownloadUrl {
    $arch = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }

    if ($Version -eq "latest") {
        $baseUrl = "https://github.com/$Repo/releases/latest/download"
    } else {
        $baseUrl = "https://github.com/$Repo/releases/download/$Version"
    }

    return "$baseUrl/${Binary}-windows-${arch}.exe"
}

function Install-Binary {
    param([string]$Url, [string]$InstallDir)

    Write-Info "Downloading $Binary from $Url..."

    $exePath = Join-Path $InstallDir "${Binary}.exe"

    try {
        Invoke-WebRequest -Uri $Url -OutFile $exePath -UseBasicParsing
    } catch {
        Write-Error-Exit "Failed to download: $_"
    }

    Write-Success "$Binary installed to $exePath"
}

function Add-ToPath {
    param([string]$Dir)

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$Dir*") {
        Write-Info "Adding $Dir to PATH..."
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$Dir", "User")
        $env:Path = "$env:Path;$Dir"
        Write-Info "PATH updated. Restart your terminal for changes to take effect."
    }
}

function Main {
    $installDir = Join-Path $env:LOCALAPPDATA $Binary
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null

    $url = Get-DownloadUrl
    Write-Info "Detected: Windows ($([System.Environment]::Is64BitOperatingSystem ? 'x86_64' : 'x86'))"

    Install-Binary -Url $url -InstallDir $installDir
    Add-ToPath -Dir $installDir

    Write-Success "$Binary installed successfully!"

    try {
        & (Join-Path $installDir "${Binary}.exe") --version 2>$null
    } catch {
        # Ignore version check errors
    }
}

Main
