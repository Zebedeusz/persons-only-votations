# Development Tools for E2E Testing

Everything needed to run the E2E tests against a local People chain.

## Layout

```
devtools/
├── runtime/                # people_rococo_runtime.wasm built by build-runtime.sh (gitignored)
├── zombienet/              # small_network.toml, generated people-chain-spec.json, zombienet.log
├── test-users-generator/   # Registers test persons on the chain, writes src/e2e/test_users.json
└── scripts/
    ├── build-runtime.sh        # Build People Rococo runtime with MaxRingSize=2, OnboardingQueuePageSize=2
    ├── setup-network.sh        # Generate chain spec, start zombienet, write chain port to .env.e2e
    ├── generate-test-users.sh  # Fetch metadata, build and run the generator
    └── run-e2e.sh              # All of the above, then start the server and run `cargo test e2e`
```

## Prerequisites

- Rust toolchain
- `zombienet` CLI: `cargo install --git https://github.com/paritytech/zombienet`
- `subxt` CLI: `cargo install subxt-cli`
- `chain-spec-builder`: `cargo install staging-chain-spec-builder`
- `polkadot`, `polkadot-parachain`, `polkadot-execute-worker`, `polkadot-prepare-worker` in `PATH`
  (from https://github.com/paritytech/polkadot-sdk/releases or built from source)
- The [individuality](https://github.com/paritytech/individuality) repository as a sibling directory
  of this project, or `INDIVIDUALITY_DIR` pointing at it. `build-runtime.sh` clones it if missing.

## Run

```bash
./scripts/run-e2e.sh                       # full workflow
./scripts/run-e2e.sh --skip-runtime-build  # runtime already built
./scripts/run-e2e.sh --skip-tests --no-cleanup   # leave the network up for manual testing
```

Individual steps, in order: `build-runtime.sh`, `setup-network.sh`, `generate-test-users.sh`,
then `cargo test e2e` from the project root. See `scripts/README.md` for options.

## How it works

- **Runtime**: `people.rs` in the individuality repo is patched to tiny ring and queue sizes so
  rings fill with two persons, built, and the WASM copied to `runtime/`. The patch is reverted
  afterwards, also when the build fails.
- **Network**: `chain-spec-builder` generates `people-chain-spec.json` from that WASM; zombienet
  spawns a `rococo-local` relay (alice, bob) and the People parachain 1004 (collator charlie).
  Zombienet assigns the collator's WebSocket port; the script reads it from the log and writes it
  into `.env.e2e` (`E2E_PEOPLE_CHAIN_ENDPOINT` and `PEOPLE_CHAIN_PORT`).
- **Users**: the generator reserves personal ids and recognizes persons through the `DummyDim`
  pallet via sudo, deriving each key from `[personal_id; 32]`, and writes `src/e2e/test_users.json`.

## Environment variables

- `INDIVIDUALITY_DIR` — individuality checkout (default: `../individuality`)
- `UPDATE_REPO=true` — `git pull` the checkout before building
- `CHAIN_ENDPOINT` — chain endpoint for the generator when `.env.e2e` is absent

## Troubleshooting

- Network does not start: `tail devtools/zombienet/zombienet.log`; check binaries in `PATH` and that
  port 9900 is free.
- User generation fails: network must be up; metadata is cached in
  `test-users-generator/people-metadata.scale`, delete it after a runtime change.
- Ring proof errors in tests: users must be *included* in rings, not only onboarding. Wait a few
  blocks after generation, or regenerate users.
- Logs: `devtools/zombienet/zombienet.log`, `server.log` (project root),
  `RUST_LOG=debug cargo test e2e -- --nocapture`.
