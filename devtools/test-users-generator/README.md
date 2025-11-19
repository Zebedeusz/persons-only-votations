# Test Users Generator

This tool generates test users for e2e tests by:
1. Reserving personal IDs via the DummyDim pallet
2. Generating Bandersnatch key pairs for each user
3. Recognizing personhood on the People chain
4. Outputting user data to `src/e2e/test_users.json`

## Requirements

- People chain running with the DummyDim pallet
- Chain metadata file: `people-metadata.scale`
- Alice account with sudo access (default)

## Generating Metadata

Before first use, generate the metadata file:

```bash
subxt metadata --url ws://127.0.0.1:9910 -f bytes > people-metadata.scale
```

Or the script will attempt to download it automatically.

## Usage

```bash
# Basic usage (connects to ws://127.0.0.1:9910, generates 6 recognized + 1 onboarding)
cargo run --release

# Custom endpoint
cargo run --release ws://127.0.0.1:12345

# Custom number of users
cargo run --release ws://127.0.0.1:9910 8 2  # 8 recognized, 2 onboarding
```

## Output

The tool generates `../../src/e2e/test_users.json` containing:
```json
{
  "recognized_persons": [
    {
      "id": 1,
      "secret_scalar": "...",
      "member_bytes": [...],
      "member_hex": "..."
    }
  ],
  "onboarding_queue": [...]
}
```

This file is used by e2e tests to create ring proofs and verify identity.
