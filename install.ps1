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
    if (-not [System.Environment]::Is64BitOperatingSystem) {
        Write-Error-Exit "32-bit Windows no soportado. SourisDW requiere 64-bit."
    }

    if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64' -or $env:PROCESSOR_IDENTIFIER -match 'ARM') {
        Write-Error-Exit "Windows ARM64 no soportado. Usa una maquina x64 o compila desde el codigo fuente."
    }

    if ($Version -eq "latest") {
        $baseUrl = "https://github.com/$Repo/releases/latest/download"
    } else {
        $baseUrl = "https://github.com/$Repo/releases/download/$Version"
    }

    return "$baseUrl/${Binary}-windows-x86_64.exe"
}

function Install-Binary {
    param([string]$Url, [string]$InstallDir)

    Write-Info "Downloading $Binary from $Url..."

    $exePath = Join-Path $InstallDir "${Binary}.exe"

    try {
        Invoke-WebRequest -Uri $Url -OutFile $exePath -UseBasicParsing -MaximumRetryCount 3
    } catch {
        Write-Error-Exit "Failed to download: $_"
    }

    Write-Success "$Binary installed to $exePath"
}

function Initialize-SourisDW {
    param([string]$InstallDir)

    $exePath = Join-Path $InstallDir "${Binary}.exe"
    try {
        & $exePath setup --quiet
    } catch {
        Write-Info "Setup warning: $_"
    }
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
    Write-Info "Detected: Windows x86_64"

    Install-Binary -Url $url -InstallDir $installDir
    Add-ToPath -Dir $installDir
    Initialize-SourisDW -InstallDir $installDir

    Write-Success "$Binary installed successfully!"

    try {
        & (Join-Path $installDir "${Binary}.exe") --version 2>$null
    } catch {
        # Ignore version check errors
    }
}

Main
