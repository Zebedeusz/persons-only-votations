#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEVTOOLS_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$DEVTOOLS_DIR")"
ZOMBIENET_DIR="$DEVTOOLS_DIR/zombienet"
INDIVIDUALITY_DIR="${INDIVIDUALITY_DIR:-$(dirname "$PROJECT_ROOT")/individuality}"

echo -e "${GREEN}Setting up E2E Test Network${NC}"
echo "==========================================="

# Check for zombienet
if ! command -v zombienet &> /dev/null; then
    echo -e "${RED}✗ zombienet not found${NC}"
    echo -e "${YELLOW}Please install zombienet:${NC}"
    echo "  cargo install --git https://github.com/paritytech/zombienet"
    echo "  Or download from: https://github.com/paritytech/zombienet/releases"
    exit 1
fi

echo -e "${GREEN}✓ zombienet found: $(command -v zombienet)${NC}"

# Check if binaries are in PATH
echo -e "${YELLOW}Checking for required binaries in PATH...${NC}"

REQUIRED_BINARIES=("polkadot" "polkadot-parachain" "polkadot-execute-worker" "polkadot-prepare-worker")
MISSING_BINARIES=()

for binary in "${REQUIRED_BINARIES[@]}"; do
    if ! command -v "$binary" &> /dev/null; then
        MISSING_BINARIES+=("$binary")
    else
        echo -e "${GREEN}✓ Found $binary: $(command -v $binary)${NC}"
    fi
done

if [ ${#MISSING_BINARIES[@]} -gt 0 ]; then
    echo ""
    echo -e "${RED}✗ Missing binaries in PATH: ${MISSING_BINARIES[*]}${NC}"
    echo ""
    echo -e "${YELLOW}To add binaries to your PATH:${NC}"
    echo ""
    echo -e "${BLUE}1. If you have binaries in a specific directory (e.g., /path/to/binaries):${NC}"
    echo ""
    echo "   Add to your shell configuration file (~/.zshrc, ~/.bashrc, or ~/.bash_profile):"
    echo ""
    echo -e "   ${GREEN}export PATH=\"/path/to/binaries:\$PATH\"${NC}"
    echo ""
    echo "   Then reload your shell:"
    echo -e "   ${GREEN}source ~/.zshrc${NC}  # or ~/.bashrc"
    echo ""
    echo -e "${BLUE}2. Download from releases:${NC}"
    echo "   https://github.com/paritytech/polkadot-sdk/releases"
    echo ""
    echo -e "${BLUE}3. Build from source:${NC}"
    echo "   git clone https://github.com/paritytech/polkadot-sdk"
    echo "   cd polkadot-sdk"
    echo "   cargo build --release"
    echo "   export PATH=\"\$(pwd)/target/release:\$PATH\""
    echo ""
    exit 1
fi

echo -e "${GREEN}✓ All required binaries found in PATH${NC}"

# Generate chain specs
echo -e "${YELLOW}Generating chain specifications...${NC}"

PEOPLE_CHAIN_SPEC="$ZOMBIENET_DIR/people-chain-spec.json"
CUSTOM_RUNTIME="$DEVTOOLS_DIR/runtime/people_rococo_runtime.wasm"

# Always generate fresh People chain spec using our custom runtime
if ! command -v chain-spec-builder &> /dev/null; then
    echo -e "${RED}✗ chain-spec-builder not found in PATH${NC}"
    echo -e "${YELLOW}Install with: cargo install staging-chain-spec-builder${NC}"
    exit 1
fi

if [ ! -f "$CUSTOM_RUNTIME" ]; then
    echo -e "${RED}✗ Custom runtime WASM not found at $CUSTOM_RUNTIME${NC}"
    echo -e "${YELLOW}Run ./build-runtime.sh first${NC}"
    exit 1
fi

if ! chain-spec-builder \
    -c "$PEOPLE_CHAIN_SPEC" \
    create \
    --para-id 1004 \
    --relay-chain rococo-local \
    -t development \
    --runtime "$CUSTOM_RUNTIME" \
    named-preset development; then
    echo -e "${RED}✗ Chain spec generation failed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ People chain spec generated from custom runtime${NC}"

# Start zombienet
echo -e "${YELLOW}Starting zombienet network...${NC}"
echo -e "${BLUE}This will run in the background. Check zombienet.log for details.${NC}"

cd "$ZOMBIENET_DIR"

# Kill any existing zombienet processes
pkill -f zombienet || true
sleep 2

# Start zombienet in the background
zombienet -p native spawn small_network.toml > zombienet.log 2>&1 &
ZOMBIENET_PID=$!

echo -e "${GREEN}✓ Zombienet started (PID: $ZOMBIENET_PID)${NC}"

# Wait for network to be ready
echo -e "${YELLOW}Waiting for network to initialize...${NC}"

MAX_WAIT=60
ELAPSED=0
while [ $ELAPSED -lt $MAX_WAIT ]; do
    if grep -q "Network launched" zombienet.log 2>/dev/null; then
        echo -e "${GREEN}✓ Network launched successfully${NC}"
        break
    fi

    if ! kill -0 $ZOMBIENET_PID 2>/dev/null; then
        echo -e "${RED}✗ Zombienet process died${NC}"
        tail -20 zombienet.log
        exit 1
    fi

    sleep 2
    ELAPSED=$((ELAPSED + 2))
    echo -n "."
done

echo ""

if [ $ELAPSED -ge $MAX_WAIT ]; then
    echo -e "${RED}✗ Timeout waiting for network${NC}"
    tail -20 zombienet.log
    exit 1
fi

# Detect Charlie's WebSocket port
echo -e "${YELLOW}Detecting Charlie's WebSocket port...${NC}"

# Look for the Direct Link line which shows the actual WebSocket URL assigned by zombienet
# Format: "Direct Link │ https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:PORT#/explorer"
CHARLIE_PORT=$(grep -A 3 "Name.*charlie" zombienet.log | grep "Direct Link" | grep -o "ws://127\.0\.0\.1:[0-9]*" | grep -o "[0-9]*$" | head -1)

if [ -z "$CHARLIE_PORT" ]; then
    CHARLIE_PORT=9910
    echo -e "${YELLOW}Could not detect port from log, using configured port: $CHARLIE_PORT${NC}"
else
    echo -e "${GREEN}✓ Detected Charlie's port: $CHARLIE_PORT${NC}"
fi

# Update .env.e2e with detected port
ENV_E2E="$PROJECT_ROOT/.env.e2e"

if [ -f "$ENV_E2E" ]; then
    # Update existing file
    sed -i.bak "s/E2E_PEOPLE_CHAIN_ENDPOINT=.*/E2E_PEOPLE_CHAIN_ENDPOINT=ws:\/\/127.0.0.1:$CHARLIE_PORT/" "$ENV_E2E"
    sed -i.bak "s/PEOPLE_CHAIN_PORT=.*/PEOPLE_CHAIN_PORT=$CHARLIE_PORT/" "$ENV_E2E"
    rm -f "$ENV_E2E.bak"
    echo -e "${GREEN}✓ Updated $ENV_E2E${NC}"
else
    echo -e "${YELLOW}⚠ .env.e2e not found, skipping update${NC}"
fi

# Show network status
echo ""
echo -e "${GREEN}=========================================="
echo -e "Network Setup Complete!"
echo -e "==========================================${NC}"
echo -e "People Chain (Charlie): ${BLUE}ws://127.0.0.1:$CHARLIE_PORT${NC}"
echo -e "Relay Chain (Alice):    ${BLUE}ws://127.0.0.1:9900${NC}"
echo -e "Zombienet PID:          ${BLUE}$ZOMBIENET_PID${NC}"
echo -e "Logs:                   ${BLUE}$ZOMBIENET_DIR/zombienet.log${NC}"
echo ""
echo -e "${YELLOW}To stop the network:${NC}"
echo -e "  kill $ZOMBIENET_PID"
echo -e "  Or: pkill -f zombienet"
echo -e "${GREEN}==========================================${NC}"
