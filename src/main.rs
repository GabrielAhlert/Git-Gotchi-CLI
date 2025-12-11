use clap::{Parser, Subcommand};
use colored::*;
use std::process::exit;

mod state;
mod git;
mod game;

use state::GameState;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Git-Gotchi
    Init,
    /// Check your pet's status
    Status,
    /// Feed the pet (Internal use)
    Feed,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            if let Err(e) = git::install_hook() {
                eprintln!("{} {}", "Error installing hook:".red(), e);
                exit(1);
            }
            // Check if state exists, otherwise create
            if GameState::load().is_err() {
                let state = GameState::new("GitBuddy");
                if let Err(e) = state.save() {
                     eprintln!("{} {}", "Error saving state:".red(), e);
                     exit(1);
                }
                println!("{}", "Git-Gotchi initialized! GitBuddy is waiting for your commits.".green());
            } else {
                 println!("{}", "Git-Gotchi is already initialized.".yellow());
            }
        }
        Commands::Status => {
            match GameState::load() {
                Ok(mut state) => {
                    // Update health based on time
                    game::check_health(&mut state);
                    // Ignore save error on status check to avoid annoyance
                    let _ = state.save();

                    let art = game::get_ascii_art(&state.stats);
                    println!("{}", art.cyan());
                    println!("Name: {}", state.name.bold());
                    println!("Level: {}", state.stats.level);
                    println!("XP: {}", state.stats.xp);
                    println!("Health: {}/100", state.stats.health);
                    println!("Hunger: {}", state.stats.hunger);
                    println!("Status: {:?}", state.stats.status);
                    println!("Streak: {} days", state.history.streak_days);
                }
                Err(_) => {
                    eprintln!("{} Run 'git-gotchi init' first.", "No pet found!".red());
                }
            }
        }
        Commands::Feed => {
             match GameState::load() {
                Ok(mut state) => {
                    match git::get_commit_stats() {
                        Ok(stats) => {
                            let lines = stats.total_lines();
                            game::update_stats(&mut state, lines);
                            
                            if let Err(e) = state.save() {
                                eprintln!("{} {}", "Error saving state:".red(), e);
                                exit(1);
                            }

                            println!("{} Fed GitBuddy with {} lines of code!", "Nom nom!".green(), lines);
                            println!("XP: {} (Level {})", state.stats.xp, state.stats.level);
                        },
                        Err(e) => {
                            // If git fails, maybe just silently fail or warn?
                            // e.g. merge commits might not have diffs depending on command
                            eprintln!("{} {}", "Could not analyze commit:".yellow(), e);
                        }
                    }
                }
                Err(_) => {
                    // Fail silently in hook if not initialized, or warn
                    // eprintln!("Git-Gotchi not initialized.");
                }
            }
        }
    }
}
