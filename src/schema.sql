CREATE TABLE players (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    games_played INTEGER DEFAULT 0,
    games_won INTEGER DEFAULT 0,
    wallet INTEGER DEFAULT 0
);
