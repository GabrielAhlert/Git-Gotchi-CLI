use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use chrono::{DateTime, Utc};

const DATA_FILE: &str = ".gitgotchi.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub name: String,
    pub birth_date: DateTime<Utc>,
    pub last_commit_date: DateTime<Utc>,
    pub stats: Stats,
    pub history: History,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub xp: u32,
    pub level: u32,
    pub health: u32,
    pub hunger: u32,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    HAPPY,
    SAD,
    HUNGRY,
    SICK,
    DEAD,
    BLOATED,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    pub total_commits: u32,
    pub streak_days: u32,
}

impl GameState {
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            name: name.to_string(),
            birth_date: now,
            last_commit_date: now,
            stats: Stats {
                xp: 0,
                level: 1,
                health: 100,
                hunger: 0,
                status: Status::HAPPY,
            },
            history: History {
                total_commits: 0,
                streak_days: 0,
            },
        }
    }

    pub fn load() -> Result<Self, io::Error> {
        if !Path::new(DATA_FILE).exists() {
             return Err(io::Error::new(io::ErrorKind::NotFound, "No save file found"));
        }
        let data = fs::read_to_string(DATA_FILE)?;
        let state: GameState = serde_json::from_str(&data)?;
        Ok(state)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(DATA_FILE, data)?;
        Ok(())
    }
}
