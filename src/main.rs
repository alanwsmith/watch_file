#![allow(unused)]
use anyhow::Result;
//use clap::{Arg, Command};
use std::path::PathBuf;
use std::sync::Arc;
use tokio;
use watchexec::Id;
use watchexec::Watchexec;
use watchexec::command::Command;
use watchexec::command::Program;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    let wx = Watchexec::default();
    let id = Id::default();
    let command = Arc::new(Command {
        program: Program::Exec {
            prog: "ls".into(),
            args: vec![],
        },
        options: Default::default(),
    });
    wx.config.on_action(move |mut action| {
        let command = command.clone();
        let job = action.get_or_create_job(id, move || command.clone());
        job.start();

        //job.restart();
        // for event in action.events.iter() {
        //     eprintln!("EVENT: {event:?}");
        // }
        //
        // if Ctrl-C is received, quit
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit();
        };
        action
    });
    //wx.config.on_action(handler)
    wx.config.pathset(["README.md"]);
    let _ = wx.main().await?;
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

    // watch the current directory

    // let matches = Command::new("runner")
    //     .arg(
    //         Arg::new("file")
    //             .index(1)
    //             .required(true)
    //             .value_parser(clap::value_parser!(PathBuf))
    //             .help("The file to watch and run when it changes."),
    //     )
    //     .get_matches();
    // if let Some(file) = matches.get_one::<PathBuf>("file") {
    //     if file.exists() {
    //         println!("Found: {}", file.display());
    //     } else {
    //         println!("Error: file does not exist");
    //     }
    // };
    // Ok(())
}
