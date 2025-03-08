# ECE421-Project1

Requirements for Project 1:

Three Poker Game to implement:
Poker Game #1: Standard five-card draw
Poker Game #6: Badugi
Poker Game #14: Texas Hold'em

# Dealer Requirements:

High-Level Server Functional Requirements (S-FRs)
* S-FR-1: Support multiple poker variants as listed in Table 1.
* S-FR-2: Betting may be supported 
* S-FR-3: Track statistics for each player.
* S-FR-4: Provide player statistics on request. 
* S-FR-5: Support 2 to 10 players per game (unless a variant supports a single player).
* S-FR-6: Use a standard deck of 52 cards, including jokers when required by the variant. (Done)
* S-FR-7: Ensure players have unique identifiers (IDs). (Done)
* S-FR-8: Number each game sequentially starting from zero (0).
* S-FR-9: Allow the game numbering to be reset to zero (0).
* S-FR-10: Resetting the game numbering must clear retained player information.


High-Level Server Performance Requirements (S-PRs)
* S-PR-1: Support at least three poker variants from Table 1 (e.g., Five-Card Draw, Texas Hold'em, Badugi).
* S-PR-2: Assume clients have infinite funds or betting tokens for simplicity.
* S-PR-3: Ensure player statistics are tracked across at least 100 server instances.
* S-PR-4: Ensure player statistics can be reported across at least 100 server instances.
* S-PR-5: Enable statistics to be reported by player ID.
* S-PR-6: Allow statistics to be reported for a given player by game number.
* S-PR-7: Allow clients to request results even if they are not actively playing.
