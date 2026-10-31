#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEVTOOLS_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$DEVTOOLS_DIR")"
RUNTIME_DIR="$DEVTOOLS_DIR/runtime"
INDIVIDUALITY_DIR="${INDIVIDUALITY_DIR:-$(dirname "$PROJECT_ROOT")/individuality}"

echo -e "${GREEN}Building E2E Test Runtime${NC}"
echo "==========================================="

# Check if individuality repository exists
if [ ! -d "$INDIVIDUALITY_DIR" ]; then
    echo -e "${YELLOW}Individuality repository not found at $INDIVIDUALITY_DIR${NC}"
    echo -e "${YELLOW}Cloning individuality repository...${NC}"

    mkdir -p "$(dirname "$INDIVIDUALITY_DIR")"
    git clone https://github.com/paritytech/individuality.git "$INDIVIDUALITY_DIR"

    echo -e "${GREEN}✓ Repository cloned${NC}"
else
    echo -e "${GREEN}✓ Individuality repository found${NC}"

    # Optional: Update to latest
    if [ "${UPDATE_REPO:-false}" = "true" ]; then
        echo -e "${YELLOW}Updating repository...${NC}"
        (cd "$INDIVIDUALITY_DIR" && git pull)
        echo -e "${GREEN}✓ Repository updated${NC}"
    fi
fi

# Navigate to runtime directory
RUNTIME_SRC_DIR="$INDIVIDUALITY_DIR/runtimes/people-rococo"

if [ ! -d "$RUNTIME_SRC_DIR" ]; then
    echo -e "${RED}✗ Runtime source not found at $RUNTIME_SRC_DIR${NC}"
    exit 1
fi

echo -e "${YELLOW}Patching runtime configuration for e2e tests...${NC}"

# Backup original configuration
PEOPLE_RS="$RUNTIME_SRC_DIR/src/people.rs"
cp "$PEOPLE_RS" "$PEOPLE_RS.backup"

# Patch the configuration for smaller ring sizes
sed -i.tmp 's/type MaxRingSize = ConstU32<[0-9]*>/type MaxRingSize = ConstU32<2>/' "$PEOPLE_RS"
sed -i.tmp 's/type OnboardingQueuePageSize = ConstU32<[0-9]*>/type OnboardingQueuePageSize = ConstU32<2>/' "$PEOPLE_RS"
rm -f "$PEOPLE_RS.tmp"

echo -e "${GREEN}✓ Configuration patched${NC}"

# Build the runtime
echo -e "${YELLOW}Building runtime (this may take a while)...${NC}"

# Under set -e a failed build would exit before restoring the patched source
if ! (cd "$INDIVIDUALITY_DIR" && cargo build --release -p people-rococo-runtime); then
    echo -e "${RED}✗ Build failed${NC}"
    mv "$PEOPLE_RS.backup" "$PEOPLE_RS"
    exit 1
fi

echo -e "${GREEN}✓ Runtime built successfully${NC}"

# Copy the WASM artifact
WASM_SRC="$INDIVIDUALITY_DIR/target/release/wbuild/people-rococo-runtime/people_rococo_runtime.compact.compressed.wasm"
WASM_DEST="$RUNTIME_DIR/people_rococo_runtime.wasm"

if [ ! -f "$WASM_SRC" ]; then
    echo -e "${RED}✗ WASM artifact not found at $WASM_SRC${NC}"
    mv "$PEOPLE_RS.backup" "$PEOPLE_RS"
    exit 1
fi

cp "$WASM_SRC" "$WASM_DEST"
echo -e "${GREEN}✓ WASM copied to $WASM_DEST${NC}"

# Generate chain specification with the new runtime
echo -e "${YELLOW}Generating chain specification...${NC}"

CHAIN_SPEC_OUTPUT="$DEVTOOLS_DIR/zombienet/people-chain-spec.json"

# Check if chain-spec-builder is available
if ! command -v chain-spec-builder &> /dev/null; then
    echo -e "${YELLOW}⚠ chain-spec-builder not found in PATH${NC}"
    echo -e "${YELLOW}⚠ Skipping chain spec generation${NC}"
    echo -e "${YELLOW}⚠ The setup-network.sh script will generate it instead${NC}"
else
    # Generate chain spec using chain-spec-builder with our custom runtime
    if chain-spec-builder \
        -c "$CHAIN_SPEC_OUTPUT" \
        create \
        --para-id 1004 \
        --relay-chain rococo-local \
        -t development \
        --runtime "$WASM_DEST" \
        named-preset development; then
        echo -e "${GREEN}✓ Chain spec generated at $CHAIN_SPEC_OUTPUT${NC}"
    else
        echo -e "${YELLOW}⚠ Chain spec generation failed${NC}"
    fi
fi

# Restore original configuration
mv "$PEOPLE_RS.backup" "$PEOPLE_RS"
echo -e "${GREEN}✓ Original configuration restored${NC}"

# Display file size
WASM_SIZE=$(du -h "$WASM_DEST" | cut -f1)
echo ""
echo -e "${GREEN}=========================================="
echo -e "Runtime build complete!"
echo -e "WASM size: $WASM_SIZE"
echo -e "Location: $WASM_DEST"
echo -e "==========================================${NC}"
