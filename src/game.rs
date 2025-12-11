use crate::state::{GameState, Status, Stats};
use chrono::{Duration, Utc};

// Constants
const XP_PER_LEVEL: u32 = 100; // Simplified leveling
const HUNGER_THRESHOLD_SAD: u32 = 24; // Hours
const HUNGER_THRESHOLD_SICK: u32 = 48;
const HUNGER_THRESHOLD_DEAD: u32 = 168; // 1 week
const BLOAT_THRESHOLD: u32 = 300; // Lines

pub fn update_stats(state: &mut GameState, lines_changed: u32) {
    let now = Utc::now();
    let hours_since_last = (now - state.last_commit_date).num_hours() as u32;

    // Check decay before feeding
    if hours_since_last > HUNGER_THRESHOLD_DEAD {
        state.stats.status = Status::DEAD;
        state.stats.health = 0;
        return; // Dead, can't feed
    }

    // Feeding Logic
    if lines_changed < 20 {
        // Snack
        state.stats.hunger = state.stats.hunger.saturating_sub(10);
        state.stats.xp += 10;
    } else if lines_changed < BLOAT_THRESHOLD {
        // Lunch
        state.stats.hunger = 0;
        state.stats.xp += 50;
        state.stats.happiness_boost(); // Conceptual
    } else {
        // Banquet (Bloated)
        state.stats.status = Status::BLOATED;
        state.stats.xp += 20; // Less XP for spaghetti
    }

    // Leveling
    let new_level = (state.stats.xp / XP_PER_LEVEL) + 1;
    if new_level > state.stats.level {
        state.stats.level = new_level;
        // Could print "Level Up!" here or return an event
    }

    // Update time
    state.last_commit_date = now;
    state.history.total_commits += 1;
    
    // Streak logic (simplified)
    if hours_since_last < 24 {
         // maintained streak
    } else if hours_since_last > 48 {
        state.history.streak_days = 0;
    }

    // Reset status if not bloated and healthy
    if state.stats.status != Status::BLOATED && state.stats.status != Status::DEAD {
        state.stats.status = Status::HAPPY;
    }
}

pub fn check_health(state: &mut GameState) {
    let now = Utc::now();
    let hours_since_last = (now - state.last_commit_date).num_hours() as u32;

    if hours_since_last > HUNGER_THRESHOLD_DEAD {
        state.stats.status = Status::DEAD;
    } else if hours_since_last > HUNGER_THRESHOLD_SICK {
        state.stats.status = Status::SICK;
    } else if hours_since_last > HUNGER_THRESHOLD_SAD {
        state.stats.status = Status::SAD;
    }
     // Else keep current status (manual updates might have set it to BLOATED)
}

pub fn get_ascii_art(stats: &Stats) -> String {
    match stats.status {
        Status::DEAD => r#"
   (X_X)
  /|   |\  R.I.P.
"#.to_string(),
        Status::SICK => r#"
   (x_x)
  /|   |\  I don't feel so good...
"#.to_string(),
         Status::SAD => r#"
   (T_T)
  /|   |\  Feed me code...
"#.to_string(),
        Status::BLOATED => r#"
   (O_O)
  /<   >\  Too much code...
"#.to_string(),
        Status::HAPPY => {
            if stats.level == 1 {
                r#"
  .---.
 /     \
 |  ?  |
 \     /
  '---'
"#.to_string()
            } else if stats.level < 5 {
                 r#"
   (o_o)
  /|   |\
"#.to_string()
            } else {
                r#"
   [^_^]
  /|   |\  MASTER CODER
"#.to_string()
            }
        },
        _ => "???".to_string()
    }
}

impl Stats {
    fn happiness_boost(&mut self) {
        // Placeholder for happiness logic
    }
}
