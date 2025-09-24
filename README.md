# Persons-only Votations

## Left To Do
- make voting more private (ZK?) - so as not to reveal one's vote
- README update,
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

## E2E Tests

### Prerequisites
1. Start a local People chain node
2. Test users must be registered and included in active rings (not just onboarding)

### Running E2E Tests

**Standard E2E tests:**
```bash
cargo test e2e
```

**With custom People chain endpoint:**
```bash
E2E_PEOPLE_CHAIN_ENDPOINT=ws://127.0.0.1:9944 cargo test e2e
```

**With debug logging:**
```bash
RUST_LOG=debug cargo test e2e -- --nocapture
```

**Specific test examples:**
```bash
# Test poll creation and voting with identity proofs
RUST_LOG=debug cargo test test_recognized_users_poll_creation_and_voting -- --nocapture

# Test basic poll operations
RUST_LOG=debug cargo test e2e::tests::test_basic_poll_creation_and_retrieval -- --nocapture
```

## Running the server

In order to start the server:
- run a local node of people chain
- specify the node's address in the config
- run the server.
