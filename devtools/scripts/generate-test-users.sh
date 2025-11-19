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
GENERATOR_DIR="$DEVTOOLS_DIR/test-users-generator"
ENV_E2E="$PROJECT_ROOT/.env.e2e"

echo -e "${GREEN}Generating Test Users${NC}"
echo "==========================================="

# Read chain endpoint from .env.e2e if it exists
if [ -f "$ENV_E2E" ]; then
    CHAIN_ENDPOINT=$(grep "E2E_PEOPLE_CHAIN_ENDPOINT=" "$ENV_E2E" | cut -d'=' -f2 || echo "ws://127.0.0.1:9910")
else
    CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://127.0.0.1:9910}"
fi

# Parse command line arguments
NUM_RECOGNIZED=6
NUM_ONBOARDING=1

while [[ $# -gt 0 ]]; do
    case $1 in
        --endpoint)
            CHAIN_ENDPOINT="$2"
            shift 2
            ;;
        --recognized)
            NUM_RECOGNIZED="$2"
            shift 2
            ;;
        --onboarding)
            NUM_ONBOARDING="$2"
            shift 2
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Usage: $0 [--endpoint <url>] [--recognized <num>] [--onboarding <num>]"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}Configuration:${NC}"
echo "  Chain endpoint: $CHAIN_ENDPOINT"
echo "  Recognized users: $NUM_RECOGNIZED"
echo "  Onboarding users: $NUM_ONBOARDING"
echo ""

# Check/generate metadata
METADATA_FILE="$GENERATOR_DIR/people-metadata.scale"

if [ ! -f "$METADATA_FILE" ]; then
    echo -e "${YELLOW}Metadata file not found, generating...${NC}"

    if ! command -v subxt &> /dev/null; then
        echo -e "${RED}✗ subxt CLI not found${NC}"
        echo "Install with: cargo install subxt-cli"
        exit 1
    fi

    subxt metadata --url "${CHAIN_ENDPOINT}" -f bytes > "$METADATA_FILE"
    echo -e "${GREEN}✓ Metadata generated${NC}"
else
    echo -e "${GREEN}✓ Metadata file exists${NC}"

    # Optional: regenerate if older than 1 day
    if [ $(find "$METADATA_FILE" -mtime +1 2>/dev/null | wc -l) -gt 0 ]; then
        echo -e "${YELLOW}Metadata is older than 1 day, consider regenerating${NC}"
    fi
fi

# Build the generator tool
echo -e "${YELLOW}Building test users generator...${NC}"

cd "$GENERATOR_DIR"
cargo build --release --quiet

if [ $? -ne 0 ]; then
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Generator built${NC}"

# Run the generator
echo -e "${YELLOW}Generating test users...${NC}"
echo ""

GENERATOR_BIN="$GENERATOR_DIR/target/release/generate-test-users"

if ! "$GENERATOR_BIN" "$CHAIN_ENDPOINT" "$NUM_RECOGNIZED" "$NUM_ONBOARDING"; then
    echo -e "${RED}✗ User generation failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}=========================================="
echo -e "Test Users Generated Successfully!"
echo -e "==========================================${NC}"
echo -e "Output: ${BLUE}$PROJECT_ROOT/src/e2e/test_users.json${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Manually trigger onboarding and ring building on the chain"
echo "  2. Or wait for automatic processing"
echo "  3. Run e2e tests: cargo test e2e"
echo -e "${GREEN}==========================================${NC}"
