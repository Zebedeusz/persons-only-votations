# Development Tools for E2E Testing

This directory contains all the tools and configurations needed to run E2E tests for the Persons-Only Votations project without external dependencies.

## Overview

The devtools directory makes this repository self-contained by providing:
- **Runtime building** - Automated build of People Rococo runtime with test configuration
- **Network setup** - Zombienet configuration for local test network
- **Test data generation** - Tool to create and register test users
- **Automation scripts** - Complete workflow orchestration

## Directory Structure

```
devtools/
├── runtime/                    # Runtime artifacts
│   ├── .gitkeep               # Empty directory marker
│   ├── README.md              # Runtime build documentation
│   └── *.wasm                 # Generated runtime (gitignored)
│
├── zombienet/                  # Network configuration
│   ├── binaries/              # Node executables (user-provided)
│   │   ├── .gitkeep
│   │   ├── README.md          # Download instructions
│   │   └── polkadot*          # Binaries (gitignored)
│   ├── small_network.toml     # Zombienet config
│   ├── README.md              # Network documentation
│   ├── zombienet.log          # Runtime logs (gitignored)
│   └── *-chain-spec.json      # Generated specs (gitignored)
│
├── test-users-generator/       # Test data generation tool
│   ├── src/
│   │   └── main.rs            # User generation logic
│   ├── Cargo.toml             # Dependencies
│   ├── README.md              # Tool documentation
│   ├── people-metadata.scale  # Chain metadata (gitignored)
│   └── target/                # Build artifacts (gitignored)
│
├── scripts/                    # Automation scripts
│   ├── build-runtime.sh       # Build test runtime
│   ├── setup-network.sh       # Start zombienet network
│   ├── generate-test-users.sh # Generate test users
│   ├── run-e2e.sh             # Complete workflow
│   └── README.md              # Script documentation
│
├── .cache/                     # Cached repositories (gitignored)
│   └── individuality/         # Cloned individuality repo
│
└── README.md                   # This file
```

## Quick Start

### Complete Workflow (Recommended)

Run everything with a single command:
```bash
./scripts/run-e2e.sh
```

### Individual Steps

For more control, run steps separately:

1. **Build Runtime**
   ```bash
   ./scripts/build-runtime.sh
   ```

2. **Setup Network**
   ```bash
   ./scripts/setup-network.sh
   ```

3. **Generate Test Users**
   ```bash
   ./scripts/generate-test-users.sh
   ```

4. **Run Tests** (from project root)
   ```bash
   cd .. && cargo test e2e
   ```

## Prerequisites

### Required Tools

Install these before running scripts:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Zombienet CLI
cargo install --git https://github.com/paritytech/zombienet

# Subxt CLI (for metadata generation)
cargo install subxt-cli
```

### Required Binaries

Download or build these binaries and place in `zombienet/binaries/`:
- `polkadot` - Relay chain node
- `polkadot-parachain` - Parachain collator
- `polkadot-execute-worker` - Worker binary
- `polkadot-prepare-worker` - Worker binary

**Download from:**
- https://github.com/paritytech/polkadot-sdk/releases

**Or build from source:**
```bash
git clone https://github.com/paritytech/polkadot-sdk
cd polkadot-sdk
cargo build --release
cp target/release/polkadot* /path/to/persons-only-votations/devtools/zombienet/binaries/
```

## How It Works

### Runtime Building

The `build-runtime.sh` script:
1. Clones the individuality repository to `.cache/individuality` (if not present)
2. Temporarily patches the runtime configuration:
   - `MaxRingSize = ConstU32<2>` (for fast ring building in tests)
   - `OnboardingQueuePageSize = ConstU32<2>` (for small test batches)
3. Builds the runtime with `cargo build --release`
4. Copies the compiled WASM to `runtime/people_rococo_runtime.wasm`
5. Restores the original configuration

### Network Setup

The `setup-network.sh` script:
1. Validates required binaries are present
2. Copies chain specifications from the cached individuality repo
3. Starts zombienet with the Rococo relay chain and two parachains
4. Detects the People parachain port (Charlie's node)
5. Updates `.env.e2e` with the detected endpoint

### Test User Generation

The `generate-test-users.sh` script:
1. Builds the test-users-generator tool
2. Fetches chain metadata (if not cached)
3. Reserves personal IDs via the DummyDim pallet
4. Generates Bandersnatch key pairs for each user
5. Recognizes personhood on-chain
6. Writes output to `../src/e2e/test_users.json`

## Configuration

### Environment Variables

Scripts respect these environment variables:

- `INDIVIDUALITY_DIR` - Path to individuality repository
  - Default: `devtools/.cache/individuality`
  - Override: `export INDIVIDUALITY_DIR=/path/to/individuality`

- `UPDATE_REPO` - Update cached repository before building
  - Default: `false`
  - Enable: `export UPDATE_REPO=true`

- `CHAIN_ENDPOINT` - Override chain endpoint
  - Default: Read from `.env.e2e` or `ws://127.0.0.1:9910`
  - Override: `export CHAIN_ENDPOINT=ws://127.0.0.1:12345`

### Script Options

See `scripts/README.md` for detailed options for each script.

## Troubleshooting

### Common Issues

**"zombienet not found"**
```bash
cargo install --git https://github.com/paritytech/zombienet
```

**"Missing binaries"**
- Download from polkadot-sdk releases
- Or build from source and copy to `zombienet/binaries/`

**"Cannot connect to chain"**
- Check zombienet is running: `ps aux | grep zombienet`
- Check logs: `cat zombienet/zombienet.log`
- Verify port is accessible: `curl http://127.0.0.1:9910`

**"Build failed"**
- Ensure Rust toolchain is up to date: `rustup update`
- Check disk space for large builds
- Review build output for specific errors

### Logs and Debugging

- **Zombienet logs**: `devtools/zombienet/zombienet.log`
- **Server logs**: `../server.log` (project root)
- **Build output**: Displayed in terminal
- **Test output**: `RUST_LOG=debug cargo test e2e -- --nocapture`

## Maintenance

### Updating the Cached Repository

To update the individuality repository cache:
```bash
export UPDATE_REPO=true
./scripts/build-runtime.sh
```

Or manually:
```bash
cd devtools/.cache/individuality
git pull
```

### Regenerating Metadata

If the People chain runtime changes:
```bash
rm devtools/test-users-generator/people-metadata.scale
./scripts/generate-test-users.sh
```

### Cleaning Build Artifacts

```bash
# Clean runtime builds
rm -rf devtools/runtime/*.wasm

# Clean test-users-generator
rm -rf devtools/test-users-generator/target/

# Clean cached repo
rm -rf devtools/.cache/

# Clean everything
git clean -fdx devtools/
```

## Architecture Decisions

### Why Cache the Individuality Repository?

The runtime source is complex with many internal dependencies. Rather than copying source code, we:
- Clone the repository once
- Build with patches applied temporarily
- Extract only the compiled WASM artifact

This approach:
- ✅ Keeps build process simple
- ✅ Stays up-to-date with individuality changes
- ✅ Avoids copying large amounts of source code
- ✅ Maintains runtime compatibility

### Why Separate Tools?

Each component (runtime, network, users) is independent:
- Test different runtime versions
- Reuse existing networks
- Generate different user sets
- Mix and match for flexibility

### Why Not Commit Binaries?

Node binaries are large (100+ MB each) and:
- Vary by platform (Linux, macOS, Windows)
- Update frequently
- Available from official releases

Users download or build binaries once, then reuse them.

## Contributing

When adding new scripts or tools:
1. Follow existing naming conventions
2. Add comprehensive README documentation
3. Include error handling and helpful messages
4. Update this README with new components
5. Test on a clean repository clone

## See Also

- `scripts/README.md` - Detailed script documentation
- `runtime/README.md` - Runtime building details
- `zombienet/README.md` - Network configuration
- `test-users-generator/README.md` - User generation details
- `../README.md` - Main project documentation
