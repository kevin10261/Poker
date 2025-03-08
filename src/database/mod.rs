//! Database module to handle player registration, login, and player statistics using SQLite.
//! 
//! This module provides functionality for player management, including:
//! - Registering new players with a unique ID and initial wallet balance.
//! - Logging in players by their username.
//! - Retrieving player statistics (games played, games won, wallet balance).
//! - Updating player statistics after a game.
//! 
//! It uses `sqlx` for asynchronous database interactions and `uuid` for unique player IDs.

use crate::lobby::Player;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use std::sync::Arc;

/// Represents a player's statistics, including games played, games won, and wallet balance.
#[derive(Debug)]
pub struct PlayerStats {
    pub id: String,
    pub name: String,
    pub games_played: i32,
    pub games_won: i32,
    pub wallet: i32,
}

/// Database wrapper that provides an interface for player management.
#[derive(Clone)]
pub struct Database {
    pub pool: Arc<SqlitePool>,
}

impl Database {
    /// Creates a new database instance with the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Database {
            pool: Arc::new(pool),
        }
    }

    /// Registers a new player with a unique ID and an initial wallet balance of 1000.
    /// 
    /// # Arguments
    /// * `name` - The player's username (must be unique).
    /// 
    /// # Returns
    /// * `Ok(String)` - The generated player ID if registration succeeds.
    /// * `Err(sqlx::Error)` - If the insertion fails (e.g., duplicate username).
    pub async fn register_player(&self, name: &str) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let wallet: u32 = 1000;
        sqlx::query("INSERT INTO players (id, name, wallet) VALUES (?1, ?2, ?3)")
            .bind(&id)
            .bind(name)
            .bind(&wallet)
            .execute(&*self.pool)
            .await?;
        Ok(id)
    }

    /// Logs in a player by checking if their username exists in the database.
    /// 
    /// # Arguments
    /// * `name` - The player's username.
    /// 
    /// # Returns
    /// * `Ok(Some(String))` - The player ID if the user exists.
    /// * `Ok(None)` - If no such user exists.
    /// * `Err(sqlx::Error)` - If a database error occurs.
    pub async fn login_player(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query("SELECT id FROM players WHERE name = ?1")
            .bind(name)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(|r| r.get(0)))
    }

    /// Retrieves a player's statistics (games played, games won, and wallet balance) by username.
    /// 
    /// # Arguments
    /// * `username` - The player's username.
    /// 
    /// # Returns
    /// * `Ok(PlayerStats)` - The player's statistics if found.
    /// * `Err(sqlx::Error)` - If the user does not exist or a database error occurs.
    pub async fn player_stats(&self, username: &str) -> Result<PlayerStats, sqlx::Error> {
        let row = sqlx::query("SELECT games_played, games_won, wallet FROM players WHERE name = ?1")
            .bind(username)
            .fetch_one(&*self.pool)
            .await?;

        Ok(PlayerStats {
            games_played: row.get(0),
            games_won: row.get(1),
            wallet: row.get(2),
            id: Uuid::new_v4().to_string(),
            name: username.to_string(),
        })
    }

    /// Retrieves the wallet balance of a player by username.
    /// 
    /// # Arguments
    /// * `username` - The player's username.
    /// 
    /// # Returns
    /// * `Ok(i32)` - The player's wallet balance if found.
    /// * `Err(sqlx::Error)` - If the user does not exist or a database error occurs.
    pub async fn get_player_wallet(&self, username: &str) -> Result<i32, sqlx::Error> {
        let row = sqlx::query("SELECT wallet FROM players WHERE name = ?1")
            .bind(username)
            .fetch_one(&*self.pool)
            .await?;
        Ok(row.get(0))
    }

    /// Updates a player's statistics (games played, games won, wallet balance) in the database.
    /// 
    /// # Arguments
    /// * `player` - A reference to the `Player` struct containing updated stats.
    /// 
    /// # Returns
    /// * `Ok(())` - If the update is successful.
    /// * `Err(sqlx::Error)` - If a database error occurs.
    pub async fn update_player_stats(&self, player: &Player) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE players SET games_played = games_played + ?1, games_won = games_won + ?2, wallet = ?3 WHERE name = ?4",
        )
        .bind(player.games_played)
        .bind(player.games_won)
        .bind(player.wallet)
        .bind(&player.name)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sets up an in-memory SQLite database for testing.
    async fn setup_database() -> Database {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE players (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                games_played INTEGER DEFAULT 0,
                games_won INTEGER DEFAULT 0,
                wallet INTEGER DEFAULT 1000
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        Database::new(pool)
    }

    /// Tests if player statistics persist correctly after multiple updates.
    #[tokio::test]
    async fn test_statistics_across_multiple_instantiations() {
        let db = setup_database().await;
        let player_name = "test_player";
        db.register_player(player_name).await.unwrap();
        
        for _ in 0..100 {
            sqlx::query("UPDATE players SET games_played = games_played + 1, games_won = games_won + 1 WHERE name = ?1")
                .bind(player_name)
                .execute(&*db.pool)
                .await
                .unwrap();
        }

        let stats = db.player_stats(player_name).await.unwrap();
        assert_eq!(stats.games_played, 100);
        assert_eq!(stats.games_won, 100);
    
}

    ///Test of testing where a statistic is reported to player using the username
    //S-FR-4
    #[tokio::test]
    async fn test_player_stats() {
        let db = setup_database().await;

        // Register a player
        let player_name = "test_player";
        db.register_player(player_name).await.unwrap();

        // Check player stats
        let stats = db.player_stats(player_name).await.unwrap();
        assert_eq!(stats.games_played, 0);
        assert_eq!(stats.games_won, 0);
        assert_eq!(stats.wallet, 1000);

    }
    /// Test to check if all players have a unique username (no duplicates)
    #[tokio::test]
    async fn test_unique_username() {
        let db = setup_database().await;

        // Register a player
        let player_name = "unique_player";
        db.register_player(player_name).await.unwrap();

        // Attempt to register another player with the same name
        let result = db.register_player(player_name).await;

        // Check that the second registration attempt fails
        assert!(result.is_err());
    }
}


