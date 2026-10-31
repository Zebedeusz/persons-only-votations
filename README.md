# Persons-only Votations

HTTP backend for yes/no polls where only verified persons can create polls and vote. Personhood is
proven against the Polkadot **People chain**; this service never holds identities of its own, it
only checks what the chain says. A voter stays anonymous towards the poll but can vote once per
poll, enforced through ring VRF aliases.

Two ballot schemes are supported:
- **Commit-reveal**: a Blake2b commitment is submitted while the poll is open, the vote and nonce are
  revealed after it ends, then the poll is finalized.
- **Encrypted ballots**: votes are TFHE-encrypted under the server's public key and summed
  homomorphically; only the total is ever decrypted.

State lives in memory. Written in Rust with axum; the chain is read through
[subxt](https://github.com/paritytech/subxt).

## How personhood is verified

A participant is identified by the hex of their Bandersnatch public key (`EncodedPublicKey`), the
same key registered on the People chain. Every poll creation and vote carries that key, a plain
signature over the request, and (for votes) a ring proof. The server checks them against the chain:

1. **Chain lookup** (`src/chain/people_multi.rs`, subxt `OnlineClient<PolkadotConfig>` with the
   metadata-generated API in `src/chain/subxt_generated_people_apis.rs`):
   - `People.Keys(key)` → the person's `PersonalId`
   - `People.People(id).position` must be `Included { ring_index, .. }` (not onboarding or suspended)
   - `People.Root(ring_index)` → the ring's `MembersCommitment`
   A person is verified only if all three exist.
2. **Signature**: `BandersnatchVrfVerifiable::verify_signature` over the canonical JSON message of
   the request (poll data for creation; `{poll_id, vote_commitment, context}` for commits;
   `{poll_id, encrypted_vote, context}` for encrypted ballots).
3. **Ring proof** (votes only): the client opens the ring from `People.RingKeys(ring_index)` and
   creates a proof with its secret over the poll's **context** (`Blake2b("PERSONS_POLL_{id}")`) and
   the same message. The server validates it against the ring root from step 1
   (`src/crypto/ring_proof.rs`). Validation yields the voter's **alias**, stable per (person,
   context), so a second ballot from the same person on the same poll is rejected without the
   server learning which ring member voted.

Once verified, `src/services/` applies the voting rules and `src/storage/` keeps polls and votes.

## Run it

```bash
cp .env.example .env     # set PEOPLE_CHAIN_PORT to your People chain's WebSocket port
cargo run
```

The server connects to `PEOPLE_CHAIN_PROTOCOL://PEOPLE_CHAIN_HOST:PEOPLE_CHAIN_PORT` (retrying with
backoff) and listens on `SERVER_HOST:SERVER_PORT`, `http://127.0.0.1:3000` by default. Check
`GET /health`. Unit tests: `cargo test --lib -- --skip e2e`.

### API

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/health` | Liveness |
| `GET` | `/polls` | List polls |
| `POST` | `/polls` | Create a poll (signed `poll_data`) |
| `GET` | `/polls/:poll_id` | Poll details |
| `POST` | `/polls/:poll_id/commit` | Commit a vote (commitment, signature, ring proof) |
| `POST` | `/polls/:poll_id/reveal` | Reveal a vote (type and nonce) after the poll ends |
| `POST` | `/polls/:poll_id/finalize` | Count revealed votes and close the poll |
| `GET` | `/polls/:poll_id/votes` | Yay/nay counts, `0/0` while the poll is open |
| `GET` | `/encryption-key` | TFHE compact public key for encrypted ballots |
| `POST` | `/polls/:poll_id/cast-encrypted` | Cast an encrypted ballot (ciphertext, signature, ring proof) |
| `GET` | `/polls/:poll_id/encrypted-votes` | Stored encrypted ballots |
| `GET` | `/polls/:poll_id/homomorphic-tally` | Decrypted total, `0/0` while the poll is open |

Errors come back as `{"error": "..."}` with 400, 401, 404 or 500.

## Voting schemes

**Commit-reveal.** Commitment = `Blake2b512(vote_byte || nonce)[..32]`, hex-encoded. Reveal is
accepted only after `ends_at`; it must open the stored commitment. Counts are hidden until the poll
ends. Each poll has its own ring context, so aliases do not link votes across polls.

**Encrypted ballots.** The client fetches the server's `CompactPublicKey` (tfhe-rs shortint,
`PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS`), encrypts `1` (yay) or `0` (nay) and submits the hex of
the bincode-serialized ciphertext. The server checks it is a well-formed ciphertext for its key,
verifies the signature and ring proof, and stores it. The tally is a plain LWE sum, decrypted once;
the 8-bit plaintext space keeps it exact up to 255 yay votes.

## Known limitations

- The server holds the TFHE secret key. Encrypted ballots are protected in transit and at rest,
  but a curious server could decrypt any single ballot.
- Encrypted ballots are not proven to encrypt 0 or 1. Malformed ciphertexts are rejected, but a
  malicious client with the public key can encrypt any value and skew the total. A sound fix needs
  a zero-knowledge proof of plaintext range, which tfhe-rs 0.5 does not offer (newer tfhe-rs
  ships `zk-pok`; that is a protocol change, not a patch).
- Tally overflow above 255 yay votes; storage is in memory and lost on restart.

## Possible evolutions

- Public and private polls
- Decentralize it: move execution and storage on chain

## E2E tests

The tests in `src/e2e/` run against a live People chain and a running server, producing real ring
proofs from chain data. They are part of `cargo test`, so without a chain they fail; filter them
out with `--skip e2e` or run them with the tooling under `devtools/`:

```bash
./devtools/scripts/run-e2e.sh
```

This builds a People Rococo runtime with `MaxRingSize=2` and `OnboardingQueuePageSize=2`, starts a
zombienet network, registers test users, starts the server and runs `cargo test e2e`.
Prerequisites and step-by-step usage are in `devtools/README.md`.

Useful variants:

```bash
./devtools/scripts/run-e2e.sh --skip-runtime-build                          # runtime already built
./devtools/scripts/run-e2e.sh --skip-runtime-build --skip-network --skip-users  # network already up
./devtools/scripts/run-e2e.sh --no-cleanup                                  # leave the network running
RUST_LOG=debug cargo test e2e -- --nocapture                                # verbose test run
cargo test test_recognized_users_poll_creation_and_voting -- --nocapture    # a single test
```

The chain port is assigned by zombienet and written to `.env.e2e` by `setup-network.sh`; the
server reads the same file when started by `run-e2e.sh`.
