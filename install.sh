#!/bin/bash
set -e

REPO="pakeelz/shlt"
INSTALL_DIR="/usr/local/bin"

# Make sure user is linux
OS="$(uname -s)"
if [ "$OS" != "Linux" ]; then
  echo "Error: Saat ini instalasi otomatis hanya mendukung sistem operasi Linux."
  exit 1
fi

# Take URL newest releases
echo "==> Mencari rilis terbaru shlt..."
DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" |
  grep "browser_download_url.*shlt-linux-x86_64.tar.gz" |
  cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
  echo "Error: Gagal menemukan file rilis untuk Linux di repositori."
  exit 1
fi

# Download and tar to temp dir
TMP_DIR=$(mktemp -d)
echo "==> Mengunduh binary shlt..."
curl -sL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR"

# Move binary to /usr/local/bin/
echo "==> Memasang shlt ke $INSTALL_DIR (memerlukan izin sudo)..."
sudo mv "$TMP_DIR/shlt" "$INSTALL_DIR/shlt"
sudo chmod +x "$INSTALL_DIR/shlt"

# cleanup
rm -rf "$TMP_DIR"

echo "==> Instalasi selesai! Jalankan 'shlt' di terminal Anda."
