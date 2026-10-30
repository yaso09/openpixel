#!/usr/bin/env bash
set -euo pipefail

repo="yaso09/openpixel"
install_dir="${OPENPIXEL_INSTALL_DIR:-$HOME/.local/bin}"
os="$(uname -s)"
arch="$(uname -m)"

case "$os/$arch" in
  Linux/x86_64|Linux/amd64) asset="openpixel-linux-x86_64.tar.gz" ;;
  Darwin/arm64|Darwin/aarch64) asset="openpixel-macos-aarch64.tar.gz" ;;
  *) echo "Unsupported platform: $os/$arch" >&2; exit 1 ;;
esac

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
url="https://github.com/$repo/releases/latest/download/$asset"
curl --fail --location --silent --show-error "$url" -o "$tmp/openpixel.tar.gz"
tar -xzf "$tmp/openpixel.tar.gz" -C "$tmp"
mkdir -p "$install_dir"
install -m 0755 "$tmp/openpixel" "$install_dir/openpixel"
install -m 0755 "$tmp/openpixel-mcp" "$install_dir/openpixel-mcp"

echo "Installed OpenPixel to $install_dir"
case ":$PATH:" in
  *":$install_dir:"*) ;;
  *) echo "Add this directory to PATH: $install_dir" ;;
esac

if command -v npx >/dev/null 2>&1; then
  if ! npx --yes skills add https://github.com/yaso09/openpixel/skills --skill openpixel-pixel-art --global --yes; then
    echo "OpenPixel tools are installed, but the agent skill could not be installed. Run the skill install command from the README to retry." >&2
  fi
else
  echo "Node.js/npm was not found, so the agent skill was skipped. Install Node.js and run: npx skills add https://github.com/yaso09/openpixel/skills --skill openpixel-pixel-art --global --yes" >&2
fi
