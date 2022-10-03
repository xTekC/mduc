#!/usr/bin/env bash
set -e

echo Installing markdup...

MOGEN_DIR=${MOGEN_DIR-"$HOME/.markdup"}
MOGEN_BIN_DIR="$MOGEN_DIR/bin"

BIN_URL="https://github.com/xTeKc/markdup/releases/download/v0.1.0-37858ebc/markdup-x86_64-apple-darwin"
BIN_PATH="$MOGEN_BIN_DIR/markdup"

# create .markdup bin dir and markdup bin if they don't exist
mkdir -p "$MOGEN_BIN_DIR"
curl -# -L "$BIN_URL" -o "$BIN_PATH"
chmod +x "$BIN_PATH"

# Store the correct profile file (i.e. .profile for Bash or .zshrc for ZSH).
case $SHELL in
*/zsh)
    PROFILE=$HOME/.zshrc
    PREF_SHELL=zsh
    ;;
*/bash)
    PROFILE=$HOME/.bashrc
    PREF_SHELL=bash
    ;;
*/fish)
    PROFILE=$HOME/.config/fish/config.fish
    PREF_SHELL=fish
    ;;
*)
    echo "markdup: could not detect shell, manually add ${MOGEN_BIN_DIR} to your PATH."
    exit 1
esac

# Only add markdup if it isn't already in PATH.
if [[ ":$PATH:" != *":${MOGEN_BIN_DIR}:"* ]]; then
    # Add the markdup directory to the path and ensure the old PATH variables remain.
    echo >> "$PROFILE" && echo "export PATH=\"\$PATH:$MOGEN_BIN_DIR\"" >> "$PROFILE"
fi

# Warn MacOS users that they may need to manually install libusb via Homebrew:
if [[ "$OSTYPE" =~ ^darwin && ! -f /usr/local/opt/libusb/lib/libusb-1.0.0.dylib ]]; then
    echo && echo "warning: libusb not found. You may need to install it manually on MacOS via Homebrew (brew install libusb)."
fi

echo && echo "Detected your preferred shell is ${PREF_SHELL} and added markdup to PATH. Run 'source ${PROFILE}' or start a new terminal session to use markdup."
