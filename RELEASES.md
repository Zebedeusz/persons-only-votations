# Release History

## v3.1.0

### Description
Cleanup and corrections of the homomorphic scheme:
- `/encryption-key` serves a TFHE compact public key instead of the secret client key
- tally no longer wraps modulo 4 (exact up to 255 yay votes); malformed ciphertexts rejected at cast
- bulletproof "validity proof" and blake2 "pedersen commitment" removed as they proved nothing about
  the ballot; `/cast-encrypted` no longer takes `validity_proof` or `vote_commitment` (breaking)
- one chain lookup path, handlers folded into routes, dead code and unused dependencies removed

## v3.0.0

### Description
Homomorphic encryption voting scheme:
- Votes remain encrypted throughout the entire tallying process
- Validity proofs ensure encrypted votes contain valid choices (0 or 1)

Personhood verification as previously.

## v2.0.0

### Description
- Personhood verification still simple: through signature check, chain look-up and ring proof validation.

Commitment-based voting with delayed result revelation:
- Hash-based vote commitments using existing Blake2b infrastructure
- Two-phase voting protocol: commit → reveal
- Time-locked result computation (no tallies during voting)

## v1.0.0

### Description
- Simple personhood verification - through signature check, chain look-up and ring proof validation.