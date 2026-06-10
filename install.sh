#!/usr/bin/env bash
set -euo pipefail

REPO="SourisCG/SourisDW"
BINARY="souris-dw"
VERSION="${VERSION:-latest}"

info() {
    printf "\033[1;34m%s\033[0m\n" "$1"
}

success() {
    printf "\033[1;32m%s\033[0m\n" "$1"
}

error() {
    printf "\033[1;31m%s\033[0m\n" "$1" >&2
    exit 1
}

detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux" ;;
        Darwin*)    echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)          error "Unsupported OS: $(uname -s)" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64" ;;
        aarch64|arm64)   echo "aarch64" ;;
        *)               error "Unsupported architecture: $(uname -m)" ;;
    esac
}

get_download_url() {
    local os="$1"
    local arch="$2"
    local version="$3"

    if [ "$version" = "latest" ]; then
        local base_url="https://github.com/${REPO}/releases/latest/download"
    else
        local base_url="https://github.com/${REPO}/releases/download/${version}"
    fi

    case "${os}-${arch}" in
        linux-x86_64)   echo "${base_url}/${BINARY}-linux-x86_64-musl" ;;
        linux-aarch64)  echo "${base_url}/${BINARY}-linux-aarch64" ;;
        macos-x86_64)   echo "${base_url}/${BINARY}-macos-x86_64" ;;
        macos-aarch64)  echo "${base_url}/${BINARY}-macos-aarch64" ;;
        windows-x86_64) echo "${base_url}/${BINARY}-windows-x86_64.exe" ;;
        *)              error "No binary available for ${os}-${arch}" ;;
    esac
}

install_binary() {
    local url="$1"
    local os="$2"
    local install_dir="$3"

    info "Downloading ${BINARY} from ${url}..."

    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "${install_dir}/${BINARY}"
    elif command -v wget &> /dev/null; then
        wget -qO "${install_dir}/${BINARY}" "$url"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    chmod +x "${install_dir}/${BINARY}"
}

main() {
    local os arch install_dir

    os=$(detect_os)
    arch=$(detect_arch)

    info "Detected: ${os} ${arch}"

    if [ "$os" = "windows" ]; then
        install_dir="${HOME}/AppData/Local/${BINARY}"
        BINARY="${BINARY}.exe"
    else
        if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
            install_dir="/usr/local/bin"
        elif [ -d "${HOME}/.local/bin" ]; then
            install_dir="${HOME}/.local/bin"
        else
            mkdir -p "${HOME}/.local/bin"
            install_dir="${HOME}/.local/bin"
        fi
    fi

    mkdir -p "$install_dir"

    local url
    url=$(get_download_url "$os" "$arch" "$VERSION")

    install_binary "$url" "$os" "$install_dir"

    if [ "$os" != "windows" ]; then
        if ! echo "$PATH" | grep -q "$install_dir"; then
            info "Add ${install_dir} to your PATH:"
            echo "  export PATH=\"${install_dir}:\$PATH\""
        fi
    fi

    success "${BINARY} installed successfully to ${install_dir}/${BINARY}"

    if [ "$os" != "windows" ]; then
        "${install_dir}/${BINARY}" --version 2>/dev/null || true
    fi
}

main "$@"
