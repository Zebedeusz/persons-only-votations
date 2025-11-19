# Persons-only Votations

## Project idea

Platform enabling creation, follow-up and management of votations/polls.
Participants verification done with personhood assurance blockchain system - People Chain.
To interact with the _People_ chain, subxt library is used.

The whole project should have its separate Bandersnatch Ring context prefix.
Then each votation should have a separate Bandersnatch Ring context,
so e.g. a votation's context would be PERSONS_POLL_{{poll_id}} = PERSONS_POLL_123456

This is a typical backend application that only communicates
with the blockchain to verify user's personhood.
Thus, in order to achieve data persistence with blockchain resilience,
at least the data layer should be handled on chain.

## Left To Do
- cleanup - retain only what's necessary,
- code simplification.

## Possible Evolutions of this project
- public and private polls
- decentralize it - put it on chain (execution, storage)

## Quick Start

### 1. Setup
```bash
cp .env.example .env
```

### 2. Configure People Chain Connection
Edit `.env` file and **ensure you specify the chain port**:
```env
PEOPLE_CHAIN_HOST=127.0.0.1
PEOPLE_CHAIN_PORT=9944  
PEOPLE_CHAIN_PROTOCOL=ws
```

### 3. Run the Server
```bash
cargo run
```

The server will start on `http://127.0.0.1:3000` by default and automatically connect to the People Chain or fall back to mock mode for development.

## E2E Tests

This repository includes everything needed to run E2E tests.

### Quick Start (Automated)

Run the complete E2E test suite with a single command:

```bash
./devtools/scripts/run-e2e.sh
```

This will automatically:
1. Build the People Rococo runtime with test configuration
2. Set up and start a local zombienet network
3. Generate test users on the People chain
4. Start the application server
5. Run all E2E tests
6. Clean up

### Prerequisites

**Required Tools:**
- [Rust toolchain](https://rustup.rs/)
- [Zombienet CLI](https://github.com/paritytech/zombienet)
- [subxt CLI](https://github.com/paritytech/subxt): `cargo install subxt-cli`

**Required Binaries in PATH:**

The following binaries must be accessible in your system PATH:
- `polkadot`
- `polkadot-parachain`
- `polkadot-execute-worker`
- `polkadot-prepare-worker`

**Quick setup if you have binaries in a directory:**
```bash
# Add to ~/.zshrc or ~/.bashrc
export PATH="/path/to/your/binaries:$PATH"
source ~/.zshrc
```

See `devtools/zombienet/binaries/README.md` for detailed setup instructions.

### Step-by-Step Manual Workflow

If you prefer to run steps individually:

#### 1. Build the Test Runtime
```bash
./devtools/scripts/build-runtime.sh
```

Builds People Rococo runtime with `MaxRingSize=2` and `OnboardingQueuePageSize=2`.

#### 2. Start the Network
```bash
./devtools/scripts/setup-network.sh
```

Starts a local zombienet network with:
- Rococo relay chain
- People parachain (port 9910)

#### 3. Generate Test Users
```bash
./devtools/scripts/generate-test-users.sh
```

Generates test users and registers them on the People chain. Output: `src/e2e/test_users.json`

#### 4. Run Tests
```bash
cargo test e2e
```

### Advanced Options

**Run tests with debug logging:**
```bash
RUST_LOG=debug cargo test e2e -- --nocapture
```

**Run specific tests:**
```bash
# Test poll creation and voting with identity proofs
cargo test test_recognized_users_poll_creation_and_voting -- --nocapture

# Test basic poll operations
cargo test e2e::tests::test_basic_poll_creation_and_retrieval -- --nocapture

# Test homomorphic encryption
cargo test e2e::homomorphic_tests -- --nocapture
```

**Customize test user generation:**
```bash
./devtools/scripts/generate-test-users.sh --recognized 10 --onboarding 2
```

**Leave network running after tests:**
```bash
./devtools/scripts/run-e2e.sh --no-cleanup
```

**Run tests against existing network:**
```bash
./devtools/scripts/run-e2e.sh --skip-runtime-build --skip-network --skip-users
```

### Repository Structure

```
devtools/
├── runtime/              # Test runtime artifacts
├── zombienet/            # Network configuration and binaries
│   ├── binaries/         # Node executables
│   ├── small_network.toml
│   └── *-chain-spec.json
├── test-users-generator/ # Tool to generate test users
└── scripts/              # Automation scripts
    ├── build-runtime.sh
    ├── setup-network.sh
    ├── generate-test-users.sh
    └── run-e2e.sh
```

## Running the server

In order to start the server:
- run a local node of people chain
- specify the node's address in the config
- run the server.
