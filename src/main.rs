#![allow(unused)]
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
            let wx = Watchexec::default();
            let id = Id::default();
            let exe_path = PathBuf::from(".").join(file_path);
            let command = Arc::new(WatchCommand {
                program: Program::Exec {
                    //   prog: file_path.to_path_buf(),
                    prog: exe_path,
                    args: vec![],
                },
                options: Default::default(),
            });
            wx.config.on_action(move |mut action| {
                let command = command.clone();
                let job = action.get_or_create_job(id, move || command.clone());
                job.start();
                // if Ctrl-C is received, quit
                if action.signals().any(|sig| sig == Signal::Interrupt) {
                    action.quit();
                };
                action
            });
            let watch_path = WatchedPath::non_recursive(file_path);
            //wx.config.pathset(vec![watch_path]);
            wx.config.pathset(vec!["example.bash"]);
            let _ = wx.main().await?;

            //println!("Found: {}", file.display());
        } else {
            println!("Error: file does not exist");
        }
    };

    Ok(())

    // let wx = Watchexec::new(|mut action| {
    //     // print any events
    //     for event in action.events.iter() {
    //         eprintln!("EVENT: {event:?}");
    //     }
    //     // if Ctrl-C is received, quit
    //     if action.signals().any(|sig| sig == Signal::Interrupt) {
    //         action.quit();
    //     }
    //     action
    // })?;
}
