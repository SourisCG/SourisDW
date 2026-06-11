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

warn() {
    printf "\033[1;33m%s\033[0m\n" "$1"
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

detect_libc() {
    if command -v ldd &>/dev/null && ldd --version 2>&1 | grep -qi "glibc"; then
        echo "glibc"
    else
        echo "musl"
    fi
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
        linux-x86_64)
            if [ "$(detect_libc)" = "glibc" ]; then
                echo "${base_url}/${BINARY}-linux-x86_64-glibc"
            else
                echo "${base_url}/${BINARY}-linux-x86_64"
            fi
            ;;
        linux-aarch64)
            if [ "$(detect_libc)" = "glibc" ]; then
                echo "${base_url}/${BINARY}-linux-aarch64-glibc"
            else
                echo "${base_url}/${BINARY}-linux-aarch64"
            fi
            ;;
        macos-x86_64)   echo "${base_url}/${BINARY}-macos-x86_64" ;;
        macos-aarch64)  echo "${base_url}/${BINARY}-macos-aarch64" ;;
        windows-x86_64) echo "${base_url}/${BINARY}-windows-x86_64.exe" ;;
        windows-aarch64) echo "${base_url}/${BINARY}-windows-arm64.exe" ;;
        *)              error "No binary available for ${os}-${arch}" ;;
    esac
}

download() {
    local url="$1"
    local dest="$2"

    if command -v curl &> /dev/null; then
        curl -fsSL --retry 3 --retry-delay 2 "$url" -o "$dest" || error "Download failed: curl returned $?"
    elif command -v wget &> /dev/null; then
        wget -q --tries=3 -O "$dest" "$url" || error "Download failed: wget returned $?"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

verify_binary() {
    local path="$1"

    if [ ! -f "$path" ]; then
        error "Binary not found at ${path}"
    fi

    if [ ! -s "$path" ]; then
        rm -f "$path"
        error "Downloaded file is empty. The URL may be wrong or the release may not exist."
    fi

    if file "$path" 2>/dev/null | grep -qi "html\|text"; then
        rm -f "$path"
        error "Downloaded file is not a binary (got HTML/text). The URL may be wrong."
    fi

    if [ "$(uname -s)" != "MINGW"* ] && [ "$(uname -s)" != "MSYS"* ] && [ "$(uname -s)" != "CYGWIN"* ]; then
        chmod +x "$path"
    fi

    if ! "$path" --version &>/dev/null; then
        warn "Warning: binary downloaded but --version check failed. It may still work."
    fi
}

add_to_path() {
    local dir="$1"
    local shell_name
    shell_name=$(basename "${SHELL:-/bin/bash}")

    local config_file=""
    case "$shell_name" in
        bash)
            if [ -f "$HOME/.bashrc" ]; then
                config_file="$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
                config_file="$HOME/.bash_profile"
            fi
            ;;
        zsh)
            config_file="$HOME/.zshrc"
            ;;
        fish)
            config_file="$HOME/.config/fish/config.fish"
            ;;
    esac

    if [ -z "$config_file" ]; then
        warn "Could not detect shell config file. Add this manually:"
        echo "  export PATH=\"${dir}:\$PATH\""
        return
    fi

    if grep -qF "$dir" "$config_file" 2>/dev/null; then
        info "PATH already configured in ${config_file}"
        return
    fi

    printf "\033[1;34mAdd %s to PATH in %s? [Y/n] \033[0m" "$dir" "$config_file"
    read -r answer </dev/tty
    answer="${answer:-Y}"

    case "$answer" in
        [Yy]*)
            if [ "$shell_name" = "fish" ]; then
                echo "set -gx PATH $dir \$PATH" >> "$config_file"
            else
                echo "" >> "$config_file"
                echo "# Added by souris-dw installer" >> "$config_file"
                echo "export PATH=\"${dir}:\$PATH\"" >> "$config_file"
            fi
            success "PATH updated in ${config_file}. Restart your shell or run:"
            echo "  source ${config_file}"
            ;;
        *)
            info "Skipped. Add manually:"
            echo "  export PATH=\"${dir}:\$PATH\""
            ;;
    esac
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

    info "Downloading from ${url}..."
    download "$url" "${install_dir}/${BINARY}"

    info "Verifying binary..."
    verify_binary "${install_dir}/${BINARY}"

    success "${BINARY} installed to ${install_dir}/${BINARY}"

    if [ "$os" != "windows" ]; then
        if ! echo "$PATH" | grep -q "$install_dir"; then
            add_to_path "$install_dir"
        else
            info "${install_dir} is already in your PATH"
        fi
    fi

    if [ "$os" != "windows" ]; then
        "${install_dir}/${BINARY}" --version
    fi
}

main "$@"
