use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

mod state;
mod git;
mod game;

#[derive(Parser)]
#[command(name = "git-gotchi")]
#[command(about = "A CLI Tamagotchi for your git repository", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Git-Gotchi for this repo
    Init,
    /// Check your pet's status
    Status,
    /// Feed your pet (used by git hook)
    Feed,
    /// Migrate Git-Gotchi to another repository
    Migrate {
        /// Path to the new repository
        path: String,
    },
    /// Delete Git-Gotchi from this repository
    Delete,
    /// Revive your pet if it has died
    Revive,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            if Path::new(".gitgotchi.json").exists() {
                println!("{}", "Git-Gotchi is already initialized here!".yellow());
                return;
            }

            println!("Welcome to Git-Gotchi! What should we name your pet?");
            print!("> ");
            std::io::stdout().flush().unwrap();
            
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();

            let state = state::GameState::new(name);
            if let Err(e) = state.save() {
                eprintln!("Error saving state: {}", e);
                return;
            }

            if let Err(e) = git::install_hook(Path::new(".")) {
                 eprintln!("Error installing hook: {}", e);
                 // Cleanup
                 let _ = fs::remove_file(".gitgotchi.json");
                 return;
            }

            println!("{} GitBuddy initialized! {} is waiting for your commits.", "Success!".green(), name);
        },
        Commands::Status => {
            match state::GameState::load() {
                Ok(mut state) => {
                    // Update health based on time
                    game::check_health(&mut state);
                    // Ignore save error on status check to avoid annoyance
                    let _ = state.save();

                    let art = game::get_ascii_art(&state.stats);
                    println!("{}", art);
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
        },
        Commands::Feed => {
             match state::GameState::load() {
                Ok(mut state) => {
                    match git::get_commit_stats() {
                        Ok(stats) => {
                            let lines = stats.total_lines();
                            game::update_stats(&mut state, lines);
                            let _ = state.save();
                            
                            println!("{} Fed GitBuddy with {} lines of code!", "Nom nom!".green(), lines);
                            
                            let art = game::get_ascii_art(&state.stats);
                            println!("{}", art);
                            
                            println!("XP: {} (Level {})", state.stats.xp, state.stats.level);
                            println!("Status: {:?}", state.stats.status);
                        },
                        Err(e) => eprintln!("Error getting git stats: {}", e),
                    }
                }
                Err(_) => {
                    // Silent fail for hook if not initialized (e.g. if user deletes json but keeps hook)
                }
            }
        },
        Commands::Migrate { path } => {
            if let Ok(state) = state::GameState::load() {
                let target_path = PathBuf::from(path);
                if !target_path.exists() || !target_path.join(".git").exists() {
                     eprintln!("{}", "Target directory must be a valid git repository.".red());
                     return;
                }

                // Copy JSON
                let target_json = target_path.join(".gitgotchi.json");
                if let Err(e) = fs::write(target_json, serde_json::to_string_pretty(&state).unwrap()) {
                    eprintln!("Error copying state: {}", e);
                    return;
                }

                // Install Hook
                if let Err(e) = git::install_hook(&target_path) {
                    eprintln!("Error installing hook in target: {}", e);
                    return;
                }

                println!("{} Pet migrated successfully to {}!", "Success!".green(), path);
                println!("Run 'git-gotchi status' in the new repository.");
                
            } else {
                 eprintln!("{}", "No pet found to migrate.".red());
            }
        },
        Commands::Delete => {
            if Path::new(".gitgotchi.json").exists() {
                println!("{}", "Are you sure you want to delete your pet? This cannot be undone. [y/N]".red().bold());
                print!("> ");
                std::io::stdout().flush().unwrap();
                
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                
                if input.trim().eq_ignore_ascii_case("y") {
                     println!("{}", game::get_goodbye_art());
                     let _ = fs::remove_file(".gitgotchi.json");
                     let _ = git::uninstall_hook(Path::new("."));
                     println!("{}", "Git-Gotchi deleted.".red());
                } else {
                    println!("Deletion cancelled.");
                }
            } else {
                eprintln!("{}", "No pet found.".red());
            }
        },
        Commands::Revive => {
             match state::GameState::load() {
                Ok(mut state) => {
                    if state.stats.status == state::Status::DEAD {
                        state.reset();
                        let _ = state.save();
                        println!("{}", game::get_reborn_art());
                        println!("{} {} has been revived from the digital afterlife!", "Miracle!".yellow(), state.name);
                    } else {
                        println!("{} is still alive! No need to revive.", state.name);
                    }
                },
                Err(_) => {
                    eprintln!("{}", "No pet found to revive.".red());
                }
             }
        }
    }
}
