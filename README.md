# Persons-only Votations

## Project idea

Platform enabling creation, follow-up and management of votations/polls.
Participants verification done with personhood assurance mechanism defined in "_individuality_" project.
To interact with _People_ chain where the _individuality_ pallets are located, subxt library is used.
The whole project should have it's separate Bandersnatch Ring context prefix.
Then each votation should have a separate Bandersnatch Ring context, 
so e.g. a votation's context would be PERSONS_POLL_{{poll_id}} = PERSONS_POLL_123456
This, in order to ensure a person can vote only once in a votation and that their vote is private.

## API

HTTP REST.

User managements necessary (login etc, my_polls, my_votes) - or maybe can be done with smth like "login with Bandersnatch blockchain key"?

- POST create_poll(creator_public_key, creator_signature, poll_data)
  - Flow:
  1. Check signature
  2. Check creator personhood
  3. Basic poll validation
  4. Create a separate context for the poll
  5. Persist poll data
- POST vote(voter_public_key, voter_signature, vote)
  1. Check signature
  2. Check voter personhood
  3. Basic validation (poll exists, is not finished etc)
  4. Cast the vote (modify poll's tally, mark the person as voted; persist all that)
- GET my_polls
- GET my_votes

## Data Model

```rust
struct Poll { // only simple majority supported
    id: u32,
    public: bool, // if true, all users can vote; if false, the creator has to specify which users can vote or invite them
    name: String,
    description: String,
    tally: Tally,
    created_at: Timestamp,
    ends_at: Timestamp,
    context: String,
}
```

```rust
struct Tally {}
```

```rust
struct Vote {
    poll_id: u32,
    user_id: u32, // or user_key?
    vote_type: VoteType,
}
```

```rust
enum VoteType {
    Yay,
    Nay,
}

```

## E2E Tests

In order to run E2E tests:
- run a local node of people chain
- specify the node's address in the config
- run the server
- run the tests.

## Running the server

In order to start the server:
- run a local node of people chain
- specify the node's address in the config
- run the server.
