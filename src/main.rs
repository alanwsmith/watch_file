use anyhow::Result;
use chrono::Local;
use clap::{arg, command};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio;
use watchexec::Id;
use watchexec::WatchedPath;
use watchexec::Watchexec;
use watchexec::command::Command as WatchCommand;
use watchexec::command::Program;
use watchexec::job::Job;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!([file_path])
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(arg!(
-q --quiet "Don't print Running/date line before each run"))
        .get_matches();
    let quiet = matches.get_flag("quiet");
    if let Some(file_path) = matches.get_one::<PathBuf>("file_path") {
        if file_path.exists() {
            clearscreen::clear().unwrap();
            if !quiet {
                println!("Watching: {}", file_path.display());
            }
            let exe_path = PathBuf::from(".").join(file_path);
            let file_string = exe_path.display().to_string();
            let wx = Watchexec::default();
            let id = Id::default();
            let command = Arc::new(WatchCommand {
                program: Program::Exec {
                    prog: exe_path,
                    args: vec![],
                },
                options: Default::default(),
            });
            wx.config.on_action_async(move |mut action| {
                if action.signals().any(|sig| sig == Signal::Interrupt) {
                    // Reminder: if you delete this Ctrl+c won't work
                    action.quit();
                };
                let command = command.clone();
                Box::new(async move {
                    clearscreen::clear().unwrap();
                    let now = Local::now();
                    let command = command.clone();
                    let job: Job = action.get_or_create_job(id, move || command.clone());
                    let start = Instant::now();
                    job.restart().await;
                    job.to_wait().await;
                    let elapsed_time = start.elapsed();
                    if !quiet {
                        println!("---------------------------------");
                        println!(
                            "Started: {}",
                            now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                        );
                        println!("Took: {}ms", elapsed_time.as_millis(),);
                    }
                    action
                })
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
