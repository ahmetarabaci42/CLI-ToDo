use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "todo", about = "Tiny ToDo CLI in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task: todo add "buy milk"
    Add { text: String },
    /// List tasks
    List,
    /// Mark a task done by id: todo done 2
    Done { id: u32 },
    /// Remove a task by id: todo rm 3
    Rm { id: u32 },
    /// Clear all tasks
    Clear,
}

fn db_path() -> Result<PathBuf> {
    let mut p = home_dir().context("Cannot find home directory")?;
    p.push(".todo-cli.json");
    Ok(p)
}

fn load() -> Result<Vec<Task>> {
    let path = db_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let tasks: Vec<Task> = serde_json::from_str(&data).context("Corrupt JSON database")?;
    Ok(tasks)
}

fn save(tasks: &[Task]) -> Result<()> {
    let path = db_path()?;
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(&path, data).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut tasks = load()?;

    match cli.command {
        Commands::Add { text } => {
            let task = Task { id: next_id(&tasks), text, done: false };
            tasks.push(task);
            save(&tasks)?;
            println!("✓ added");
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("(no tasks)");
            } else {
                for t in &tasks {
                    let mark = if t.done { "✔" } else { " " };
                    println!("{:>3}. [{}] {}", t.id, mark, t.text);
                }
            }
        }
        Commands::Done { id } => {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                t.done = true;
                save(&tasks)?;
                println!("✓ marked as done");
            } else {
                println!("no task with id {id}");
            }
        }
        Commands::Rm { id } => {
            let len_before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < len_before {
                save(&tasks)?;
                println!("✓ removed");
            } else {
                println!("no task with id {id}");
            }
        }
        Commands::Clear => {
            tasks.clear();
            save(&tasks)?;
            println!("✓ cleared");
        }
    }

    Ok(())
}
