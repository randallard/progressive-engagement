use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a new game session
    New {
        /// Player name
        name: String,
    },
    /// Load existing game data
    Load {
        /// Path to backup file
        #[arg(short, long)]
        file: std::path::PathBuf,
    },
    /// Backup current progress
    Backup {
        /// Email to send backup to
        #[arg(short, long)]
        email: Option<String>,
    },
}