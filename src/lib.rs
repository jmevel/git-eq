use std::{
    path::Path,
    process::{Command, Output},
    time::{SystemTime, SystemTimeError},
};

use anyhow::Result;
use config::Config;

pub mod config;
mod file_watcher;

/// To run the earthquake procedure
pub fn earthquake_procedure(config: Config) -> Result<()> {
    let current_branch = current_branch()?;
    let remote = remote(&current_branch)?;
    let user_email = user_email()?;
    let elapsed = current_unix_epoch()?;
    let branch_name = format!("earthquake/{current_branch}-{user_email}-{elapsed}");

    checkout(&branch_name)?;
    if any_uncommited_changes()? {
        add()?;
        commit(&config.commit_message)?;
    }
    push(&branch_name, &remote)?;

    Ok(())
}

fn any_uncommited_changes() -> Result<bool> {
    let output = output_git_command(&["status", "--porcelain"])?;
    Ok(!output.is_empty())
}

fn current_branch() -> Result<String> {
    output_git_command(&["branch", "--show-current"])
}

fn user_email() -> Result<String> {
    let output = output_git_command(&["config", "--get", "user.email"])?;
    if output.is_empty() {
        return Err(anyhow::Error::msg("No user email is configured"));
    }
    Ok(output)
}

fn remote(branch_name: &str) -> Result<String> {
    let config_path = format!("branch.{branch_name}.remote");
    let output = output_git_command(&["config", "--get", &config_path])?;
    if output.is_empty() {
        return Err(anyhow::Error::msg("No remote branch is configured"));
    }
    Ok(output)
}

fn current_unix_epoch() -> Result<u64, SystemTimeError> {
    let current_unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(current_unix_epoch.as_secs())
}

fn checkout(branch_name: &str) -> Result<()> {
    spawn_git_command(&["checkout", "-b", branch_name])
}

fn add() -> Result<()> {
    // Add all files even when the current directory is not the root directory of the git repository
    spawn_git_command(&["add", "--all"])
}

fn commit(message: &str) -> Result<()> {
    // Bypass GPG-sign and pre-commit hook
    spawn_git_command(&["commit", "--no-gpg-sign", "--no-verify", "-m", message])
}

fn push(branch_name: &str, remote: &str) -> Result<()> {
    spawn_git_command(&["push", "-u", remote, branch_name])
}

fn spawn_git_command(args: &[&str]) -> Result<()> {
    wait_git_lock_released()?;
    let mut child = Command::new("git").args(args).spawn()?;
    child.wait()?;
    Ok(())
}

fn output_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    read_git_info(output)
}

fn wait_git_lock_released() -> Result<()> {
    let git_root_directory = output_git_command(&["rev-parse", "--show-toplevel"])?;
    let lock_path = Path::new(&git_root_directory).join(".git/index.lock");

    if lock_path.exists() {
        file_watcher::wait_until_deleted(&lock_path);
    }
    Ok(())
}

fn read_git_info(output: Output) -> Result<String> {
    let info = String::from_utf8(output.stdout)?;
    Ok(info.trim_end().to_string())
}
