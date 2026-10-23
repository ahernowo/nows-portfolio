#!/usr/bin/env bash
# One-time setup for macOS and Linux (and Git Bash on Windows, when Rust,
# Zola and Node are already installed). Installs the factory and newpost
# tools and prepares the factory board.
#
#   bash scripts/setup.sh            # everything
#   bash scripts/setup.sh --no-browser   # skip the Chromium download
#
# Safe to run again.

set -euo pipefail

ZOLA_VERSION="0.23.6"
REPO="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO"

step() { printf '\n== %s\n' "$1"; }
ok() { printf '   ok: %s\n' "$1"; }
has() { command -v "$1" >/dev/null 2>&1; }

SKIP_BROWSER=0
[ "${1:-}" = "--no-browser" ] && SKIP_BROWSER=1

export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

step "Rust"
if ! has rustup; then
  echo "   installing rustup (https://rustup.rs)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
  export PATH="$HOME/.cargo/bin:$PATH"
fi
rustup default stable >/dev/null
rustup component add rustfmt clippy >/dev/null
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 \
  || echo "   note: could not add the wasm32 target; workers won't build locally until 'rustup target add wasm32-unknown-unknown' works"
ok "$(rustc --version)"

step "Zola $ZOLA_VERSION"
if has zola && zola --version | grep -q "$ZOLA_VERSION"; then
  ok "$(zola --version)"
else
  case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) target="x86_64-unknown-linux-gnu" ;;
    Linux-aarch64) target="aarch64-unknown-linux-gnu" ;;
    Darwin-arm64) target="aarch64-apple-darwin" ;;
    Darwin-x86_64) target="x86_64-apple-darwin" ;;
    *) echo "   Install Zola $ZOLA_VERSION by hand: https://www.getzola.org/documentation/getting-started/installation/"; exit 1 ;;
  esac
  mkdir -p "$HOME/.local/bin"
  curl -fsSL "https://github.com/getzola/zola/releases/download/v${ZOLA_VERSION}/zola-v${ZOLA_VERSION}-${target}.tar.gz" | tar -xz -C "$HOME/.local/bin"
  ok "$(zola --version) in ~/.local/bin (add it to your PATH if it isn't)"
fi

step "Node.js"
if has node; then ok "node $(node --version)"; else echo "   Install Node.js 22 LTS from https://nodejs.org, then run this again."; exit 1; fi

step "Project tools: factory and newpost"
cargo install --path tools/factory --locked --quiet
cargo install --path tools/newpost --locked --quiet
ok "$(factory --version)"
ok "$(newpost --version)"

step "Factory board"
factory init
factory status

step "Screenshot tool"
(cd tools/shots && npm install --no-audit --no-fund && { [ "$SKIP_BROWSER" = 1 ] || npx playwright install chromium; })
ok "tools/shots ready"

step "First build"
zola --root site build
cargo test --workspace --quiet

printf '\nAll set. Next: docs/setup.md, step 3 (preview the site).\n'
