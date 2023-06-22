use clap::{Parser, arg, command};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the crate to analyze
    #[arg(short, long)]
    pub name: String,
}