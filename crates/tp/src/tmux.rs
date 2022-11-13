use anyhow::Result;
use std::option::Option;
use std::process::{Command, Stdio};

pub fn create_session(path: &str, session: &str) -> Result<()> {
    let output = Command::new("tmux")
        .arg("new-session")
        .arg("-d")
        .arg("-c")
        .arg(path)
        .arg("-s")
        .arg(session)
        .stdout(Stdio::null())
        .output()?;

    if output.status.success() {
        Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg(session)
            .arg("nvim")
            .arg("ENTER")
            .output()?;
    }

    let tmux_running: Option<&'static str> = option_env!("TMUX");

    if tmux_running.is_some() {
        Command::new("tmux")
            .arg("switch-client")
            .arg("-t")
            .arg(session)
            .output()?;
    } else {
        Command::new("tmux")
            .arg("attach")
            .arg("-t")
            .arg(session)
            .output()?;
    }

    Ok(())
}
