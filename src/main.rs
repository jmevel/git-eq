//! # `git-eq` (aka *git earthquake*)
//! 
//! Earthquakes are part of the daily life in many countries like in Taiwan.
//! `git-eq` is a simple git command to quickly save your local changes in case of an emergency like this.
//! This project is heavily inspired by [git-fire](https://github.com/qw3rtman/git-fire).
//! 
//! ## What this command does
//! 
//! 1. `Checkout` to a new branch named `earthquake/<origin-branch>-<email>-<elapsed-seconds-since-unix-epoch>` (eg: *`earthquake/master-bob@domain.com-1652438295`*)
//! 2. If there are some uncommited changes
//!    1. `Add` all those files (even if you're not in the root directory)
//!    2. `Commit` with either the default message or the provided one
//! 3. `Push`

use std::{error::Error, env};

use git_eq::{config::Config, earthquake_procedure};

/// Run the command with an optional commit message
/// Example:
/// 
/// ```sh
/// git eq "My custom message"
/// ```
/// If no message is provided, the default `Earthquake!!! This is an emergency commit` message is used
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new(env::args());
    earthquake_procedure(config)?;
    println!("\r\nLocal changes saved, now go hide in a corner!!! (don't stay in the middle of the room)\r\n");
    Ok(())
}
