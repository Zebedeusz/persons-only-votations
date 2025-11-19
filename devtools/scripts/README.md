# E2E Test Automation Scripts

This directory contains scripts to automate the complete E2E testing workflow.

## Quick Start

Run the complete E2E test suite:
```bash
./run-e2e.sh
```

This will:
1. Build the test runtime
2. Set up the network with zombienet
3. Generate test users
4. Start the application server
5. Run E2E tests
6. Clean up

## Individual Scripts

### build-runtime.sh

Builds the People Rococo runtime with e2e test configuration.

**Usage:**
```bash
./build-runtime.sh
```

**Environment Variables:**
- `INDIVIDUALITY_DIR` - Path to individuality repository (default: `../devtools/.cache/individuality`)
- `UPDATE_REPO` - Set to `true` to update the repository before building

**What it does:**
1. Clones individuality repository if not present
2. Temporarily patches runtime configuration (`MaxRingSize=2`, `OnboardingQueuePageSize=2`)
3. Builds the runtime with `cargo build --release`
4. Copies WASM artifact to `devtools/runtime/`
5. Restores original configuration

**Output:**
- `devtools/runtime/people_rococo_runtime.wasm`

---

### setup-network.sh

Sets up and starts the zombienet test network.

**Usage:**
```bash
./setup-network.sh [--download-binaries]
```

**Options:**
- `--download-binaries` - Attempt to download required binaries (not yet implemented)

**Requirements:**
- `zombienet` CLI tool
- Node binaries in `devtools/zombienet/binaries/`:
  - `polkadot`
  - `polkadot-parachain`
  - `polkadot-execute-worker`
  - `polkadot-prepare-worker`

**What it does:**
1. Validates required binaries are present
2. Copies chain specifications from individuality repo
3. Starts zombienet network in the background
4. Detects Charlie's WebSocket port
5. Updates `.env.e2e` with detected port

**Output:**
- Network running in background
- Zombienet PID
- Log file: `devtools/zombienet/zombienet.log`
- Updated `.env.e2e`

---

### generate-test-users.sh

Generates test users on the running People chain.

**Usage:**
```bash
./generate-test-users.sh [options]
```

**Options:**
- `--endpoint <url>` - Chain WebSocket endpoint (default: from `.env.e2e`)
- `--recognized <num>` - Number of recognized persons (default: 6)
- `--onboarding <num>` - Number of onboarding persons (default: 1)

**Requirements:**
- Running People chain
- `subxt` CLI for metadata generation
- Rust toolchain for building the generator

**What it does:**
1. Checks chain connectivity
2. Generates/validates chain metadata
3. Builds the test-users-generator tool
4. Reserves personal IDs on chain
5. Generates Bandersnatch key pairs
6. Recognizes personhood via DummyDim pallet
7. Writes output to `src/e2e/test_users.json`

**Output:**
- `src/e2e/test_users.json` with test user keys
- `devtools/test-users-generator/people-metadata.scale`

---

### run-e2e.sh

Orchestrates the complete E2E test workflow.

**Usage:**
```bash
./run-e2e.sh [options]
```

**Options:**
- `--skip-runtime-build` - Skip runtime build step
- `--skip-network` - Skip network setup (use existing)
- `--skip-users` - Skip test user generation
- `--skip-tests` - Skip running tests (setup only)
- `--no-cleanup` - Don't stop network after tests
- `--help` - Show help message

**Examples:**
```bash
# Full workflow with cleanup
./run-e2e.sh

# Skip runtime build (if already built)
./run-e2e.sh --skip-runtime-build

# Setup only, leave network running for manual testing
./run-e2e.sh --skip-tests --no-cleanup

# Run tests against existing setup
./run-e2e.sh --skip-runtime-build --skip-network --skip-users
```

**What it does:**
1. Builds runtime (unless skipped)
2. Sets up network (unless skipped)
3. Generates test users (unless skipped)
4. Builds and starts application server
5. Runs `cargo test e2e`
6. Stops server and network (unless --no-cleanup)

## Troubleshooting

### Network fails to start

Check the zombienet log:
```bash
tail -f devtools/zombienet/zombienet.log
```

Common issues:
- Missing binaries in `devtools/zombienet/binaries/`
- Port conflicts (9900, 9910, 9920 must be free)
- Incorrect chain specifications

### Test user generation fails

- Ensure network is running: `ps aux | grep zombienet`
- Check chain connectivity: `curl -X POST http://127.0.0.1:9910 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}'`
- Verify DummyDim pallet is available in the runtime

### Tests fail with ring proof errors

- Ensure test users have been onboarded to rings
- Check that ring building has completed on chain
- Verify `test_users.json` contains valid keys

### Permission denied errors

Make scripts executable:
```bash
chmod +x devtools/scripts/*.sh
```

## Environment Variables

Scripts respect these environment variables:

- `INDIVIDUALITY_DIR` - Path to individuality repository cache
- `CHAIN_ENDPOINT` - Override chain endpoint
- `UPDATE_REPO` - Set to `true` to update cached repository

## Logs and Output

- Zombienet: `devtools/zombienet/zombienet.log`
- Server: `server.log` (in project root)
- Test users: `src/e2e/test_users.json`
- Runtime build: Terminal output
