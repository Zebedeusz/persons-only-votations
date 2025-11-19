# Zombienet Configuration for E2E Tests

This directory contains the configuration for running a local test network with:
- Rococo relay chain (2 validators: alice, bob)
- People parachain (ID 1004, collator: charlie)
- Asset Hub parachain (ID 1000, collator: dave)

## Files

- `small_network.toml` - Zombienet network configuration
- `people-chain-spec.json` - Chain specification for People parachain (auto-generated)
- `asset-hub-chain-spec.json` - Chain specification for Asset Hub parachain (auto-generated)
- `binaries/` - (Deprecated) Binaries should be in your system PATH instead

## Usage

The network is automatically started by the `setup-network.sh` script, but you can also run it manually:

```bash
cd devtools/zombienet
zombienet -p native spawn small_network.toml
```

## Chain Spec Files

The chain spec files will be automatically copied from the individuality repository when running `setup-network.sh`. If you need to regenerate them manually:

```bash
# From individuality repository
./target/release/polkadot-parachain build-spec \
  --chain people-rococo \
  --disable-default-bootnode \
  > people-chain-spec.json
```

## Port Configuration

- Alice (relay): 9900
- Charlie (People parachain): 9910 (used for e2e tests)
- Dave (Asset Hub): 9920

The `setup-network.sh` script will automatically detect Charlie's WebSocket port and update the `.env.e2e` file.
