#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"

echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   E2E Test Runner                      ║${NC}"
echo -e "${GREEN}║   Persons-Only Votations               ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""

# Parse command line arguments
SKIP_RUNTIME_BUILD=false
SKIP_NETWORK_SETUP=false
SKIP_USER_GENERATION=false
SKIP_TESTS=false
CLEANUP_ON_EXIT=true

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-runtime-build)
            SKIP_RUNTIME_BUILD=true
            shift
            ;;
        --skip-network)
            SKIP_NETWORK_SETUP=true
            shift
            ;;
        --skip-users)
            SKIP_USER_GENERATION=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --no-cleanup)
            CLEANUP_ON_EXIT=false
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --skip-runtime-build   Skip building the runtime"
            echo "  --skip-network         Skip network setup (use existing)"
            echo "  --skip-users           Skip test user generation"
            echo "  --skip-tests           Skip running tests"
            echo "  --no-cleanup           Don't stop network after tests"
            echo "  --help                 Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Cleanup function
cleanup() {
    if [ "$CLEANUP_ON_EXIT" = true ]; then
        echo ""
        echo -e "${YELLOW}Cleaning up...${NC}"
        pkill -f zombienet || true
        echo -e "${GREEN}✓ Network stopped${NC}"
    else
        echo ""
        echo -e "${YELLOW}Network left running${NC}"
        echo -e "${YELLOW}To stop: pkill -f zombienet${NC}"
    fi
}

trap cleanup EXIT

# Step 1: Build Runtime
if [ "$SKIP_RUNTIME_BUILD" = false ]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Step 1/5: Building Runtime${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if ! "$SCRIPT_DIR/build-runtime.sh"; then
        echo -e "${RED}✗ Runtime build failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⊘ Skipping runtime build${NC}"
fi

# Step 2: Setup Network
if [ "$SKIP_NETWORK_SETUP" = false ]; then
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Step 2/5: Setting Up Network${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if ! "$SCRIPT_DIR/setup-network.sh"; then
        echo -e "${RED}✗ Network setup failed${NC}"
        exit 1
    fi

    # Wait additional time for network stabilization
    echo -e "${YELLOW}Waiting for network to stabilize...${NC}"
    sleep 10
else
    echo -e "${YELLOW}⊘ Skipping network setup${NC}"
fi

# Step 3: Generate Test Users
if [ "$SKIP_USER_GENERATION" = false ]; then
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Step 3/5: Generating Test Users${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if ! "$SCRIPT_DIR/generate-test-users.sh"; then
        echo -e "${RED}✗ Test user generation failed${NC}"
        exit 1
    fi

    # Wait for onboarding and ring building
    echo -e "${YELLOW}Waiting for onboarding and ring building...${NC}"
    echo -e "${YELLOW}(This depends on chain configuration)${NC}"
    sleep 15
else
    echo -e "${YELLOW}⊘ Skipping test user generation${NC}"
fi

# Step 4: Start Application Server
if [ "$SKIP_TESTS" = false ]; then
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Step 4/5: Starting Application Server${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Load environment
    if [ -f "$PROJECT_ROOT/.env.e2e" ]; then
        export $(grep -v '^#' "$PROJECT_ROOT/.env.e2e" | xargs)
    fi

    # Build the server
    echo -e "${YELLOW}Building server...${NC}"
    (cd "$PROJECT_ROOT" && cargo build --release)

    # Start server in background
    echo -e "${YELLOW}Starting server...${NC}"
    "$PROJECT_ROOT/target/release/persons-only-votations" > "$PROJECT_ROOT/server.log" 2>&1 &
    SERVER_PID=$!

    echo -e "${GREEN}✓ Server started (PID: $SERVER_PID)${NC}"

    # Wait for server to be ready
    echo -e "${YELLOW}Waiting for server to be ready...${NC}"
    MAX_WAIT=30
    ELAPSED=0
    while [ $ELAPSED -lt $MAX_WAIT ]; do
        if curl -s http://127.0.0.1:3000/health > /dev/null 2>&1; then
            echo -e "${GREEN}✓ Server is ready${NC}"
            break
        fi

        if ! kill -0 $SERVER_PID 2>/dev/null; then
            echo -e "${RED}✗ Server process died${NC}"
            tail -20 "$PROJECT_ROOT/server.log"
            exit 1
        fi

        sleep 1
        ELAPSED=$((ELAPSED + 1))
    done

    if [ $ELAPSED -ge $MAX_WAIT ]; then
        echo -e "${RED}✗ Timeout waiting for server${NC}"
        kill $SERVER_PID || true
        exit 1
    fi
fi

# Step 5: Run E2E Tests
if [ "$SKIP_TESTS" = false ]; then
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Step 5/5: Running E2E Tests${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    cd "$PROJECT_ROOT"

    if cargo test e2e -- --nocapture; then
        TEST_RESULT="PASSED"
        TEST_COLOR="$GREEN"
    else
        TEST_RESULT="FAILED"
        TEST_COLOR="$RED"
    fi

    # Stop server
    if [ -n "${SERVER_PID:-}" ]; then
        kill $SERVER_PID || true
        echo -e "${GREEN}✓ Server stopped${NC}"
    fi

    # Display results
    echo ""
    echo -e "${TEST_COLOR}╔════════════════════════════════════════╗${NC}"
    echo -e "${TEST_COLOR}║   E2E Tests: $TEST_RESULT                     ║${NC}"
    echo -e "${TEST_COLOR}╚════════════════════════════════════════╝${NC}"

    if [ "$TEST_RESULT" = "FAILED" ]; then
        exit 1
    fi
else
    echo -e "${YELLOW}⊘ Skipping tests${NC}"
fi

echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   All Steps Completed Successfully!   ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
