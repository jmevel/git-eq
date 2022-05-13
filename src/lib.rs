use std::{process::{Command, Output}, error::Error, time::{SystemTime, SystemTimeError}, fs, path::Path};

use config::Config;

pub mod config;

/// To run the earthquake procedure
pub fn earthquake_procedure(config: Config) -> Result<(), Box<dyn Error>> {
    let current_branch = current_branch()?;
    let user_email = user_email()?;
    let elapsed = current_unix_epoch()?;
    let branch_name = format!("earthquake/{current_branch}-{user_email}-{elapsed}");

    checkout(&branch_name)?;
    if any_uncommited_changes()? {
        add()?;
        commit(&config.commit_message)?;
    }
    push()?;

    Ok(())
}

fn any_uncommited_changes() -> Result<bool, Box<dyn Error>> {
    let output = Command::new("git").args(&["status", "--porcelain"]).output()?;
    let output = String::from_utf8(output.stdout)?;
    Ok(!output.is_empty())
}

fn current_branch() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(&["branch", "--show-current"]).output()?;
    read_git_info(output)
}

fn user_email() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(&["config", "--get", "user.email"]).output()?;
    read_git_info(output)
}

fn current_unix_epoch() -> Result<u64, SystemTimeError> {
    let current_unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(current_unix_epoch.as_secs())
}

fn checkout(branch_name: &str) -> Result<(), Box<dyn Error>> {
    delete_index_lock()?;
    Command::new("git").args(&["checkout", "-b", branch_name]).spawn()?;
    Ok(())
}

fn add() -> Result<(), Box<dyn Error>> {
    delete_index_lock()?;
    Command::new("git").args(&["add", "--all"]).spawn()?;
    Ok(())
}

fn commit(message: &str) -> Result<(), Box<dyn Error>> {
    delete_index_lock()?;
    Command::new("git").args(&["commit", "-m", message]).spawn()?;
    Ok(())
}

fn push() -> Result<(), Box<dyn Error>> {
    delete_index_lock()?;
    Command::new("git").arg("push").spawn()?;
    Ok(())
}

fn delete_index_lock() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").args(&["rev-parse", "--show-toplevel"]).output()?;
    let git_root_directory = read_git_info(output)?;
    let lock_path = Path::new(&git_root_directory).join(".git/index.lock");
    if lock_path.exists() {
        fs::remove_file(lock_path)?;
    }
    Ok(())
}

fn read_git_info(output: Output) -> Result<String, Box<dyn Error>> {
    let mut info = String::from_utf8(output.stdout)?;
    trim_newline(&mut info);
    Ok(info)
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
