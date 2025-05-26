use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;
use std::sync::Arc;
use tokio;
use watchexec::Id;
use watchexec::WatchedPath;
use watchexec::Watchexec;
use watchexec::command::Command as WatchCommand;
use watchexec::command::Program;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("watcher")
        .arg(
            Arg::new("file_path")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("The file to watch and run when it changes."),
        )
        .get_matches();
    if let Some(file_path) = matches.get_one::<PathBuf>("file_path") {
        if file_path.exists() {
            let file_string = file_path.display().to_string();
            println!("Watching: {}", &file_string);
            let wx = Watchexec::default();
            let id = Id::default();
            let exe_path = PathBuf::from(".").join(file_path);
            let command = Arc::new(WatchCommand {
                program: Program::Exec {
                    prog: exe_path,
                    args: vec![],
                },
                options: Default::default(),
            });
            wx.config.on_action(move |mut action| {
                println!("Running: {}", file_string);
                let command = command.clone();
                let job = action.get_or_create_job(id, move || command.clone());
                job.start();
                if action.signals().any(|sig| sig == Signal::Interrupt) {
                    // Reminder: if you delete this Ctrl+c won't work
                    action.quit();
                };
                action
            });
            let watch_path = WatchedPath::non_recursive(file_path);
            wx.config.pathset(vec![watch_path]);
            let _ = wx.main().await?;
        } else {
            println!("Error: file does not exist");
        }
    };
    Ok(())
}
