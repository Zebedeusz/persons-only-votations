# Release History

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