#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use chrono::Local;
use clap::{arg, command};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use watchexec::Id;
use watchexec::WatchedPath;
use watchexec::Watchexec;
use watchexec::command::Command as WatchCommand;
use watchexec::command::Program;
use watchexec::command::Shell;
use watchexec::job::Job;
use watchexec_signals::Signal;

struct Runner {
    quiet: bool,
    requested_path: PathBuf,
}

impl Runner {
    pub fn command(&self) -> Result<String> {
        let base = self.script_name()?;
        let result = format!("./{}", base);
        Ok(result)
    }

    pub fn new() -> Runner {
        let matches = command!()
            .arg(
                arg!([file_path])
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf)),
            )
            .arg(arg!(
    -q --quiet "Don't print Running/date line before each run"))
            .get_matches();
        let requested_path = matches
            .get_one::<PathBuf>("file_path")
            .unwrap()
            .to_path_buf();
        if !requested_path.exists() {
            eprintln!("ERROR: {} does not exist", requested_path.display());
            std::process::exit(1);
        }
        Runner {
            quiet: matches.get_flag("quiet"),
            requested_path,
        }
    }

    pub async fn run(&self) -> Result<()> {
        dbg!(&self.script_name());
        let wx = Watchexec::default();
        let id = Id::default();
        let watch_command = self.watch_command();
        wx.config.on_action_async(move |mut action| {
            let watch_command = watch_command.clone();
            Box::new(async move {
                let job: Job = action.get_or_create_job(id, move || watch_command.clone());
                if action.signals().any(|sig| sig == Signal::Interrupt) {
                    // Reminder: Ctrl+c won't work if you delete `action.quite()`
                    action.quit();
                } else {
                    job.restart().await;
                    job.to_wait().await;
                };
                action
            })
        });
        let watch_path = WatchedPath::non_recursive(".");
        wx.config.pathset(vec![watch_path]);
        wx.main().await?;
        Ok(())
    }

    pub fn script_name(&self) -> Result<String> {
        let result = self
            .requested_path
            .file_name()
            .ok_or_else(|| anyhow!("Could not get script name"))?
            .to_string_lossy()
            .to_string();
        Ok(result)
    }

    pub fn watch_command(&self) -> Arc<WatchCommand> {
        Arc::new(WatchCommand {
            program: Program::Shell {
                shell: Shell::new("bash"),
                command: self.command().unwrap(),
                args: vec![],
            },
            options: Default::default(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let runner = Runner::new();
    runner.run().await?;

    //if let Some(file_path) = matches.get_one::<PathBuf>("file_path") {
    //    if file_path.exists() {
    //        if let Some(file_name) = file_path.file_name() {
    //            dbg!(file_name);
    //        }
    //        let wx = Watchexec::default();
    //        let id = Id::default();
    //        //let exe_path = PathBuf::from(".").join(file_path);
    //        let exe_path = file_path.to_path_buf();
    //        let command = Arc::new(WatchCommand {
    //            program: Program::Shell {
    //                shell: Shell::new("bash"),
    //                command: exe_path.display().to_string(),
    //                // prog: "pwd".into(),
    //                // prog: exe_path.clone(),
    //                args: vec![],
    //            },
    //            // program: Program::Exec {
    //            //     // prog: "pwd".into(),
    //            //     prog: exe_path.clone(),
    //            //     args: vec![],
    //            // },
    //            options: Default::default(),
    //        });
    //        clearscreen::clear().unwrap();
    //        if !quiet {
    //            println!("Watching: {}", file_path.display());
    //        }
    //        wx.config.on_action_async(move |mut action| {
    //            let command = command.clone();
    //            let exe_path = exe_path.clone();
    //            Box::new(async move {
    //                let command = command.clone();
    //                let job: Job = action.get_or_create_job(id, move || command.clone());
    //                if action.signals().any(|sig| sig == Signal::Interrupt) {
    //                    // Reminder: Ctrl+c won't work if
    //                    // you delete `action.quite()`
    //                    action.quit();
    //                } else {
    //                    clearscreen::clear().unwrap();
    //                    // dbg!(exe_path.display());
    //                    let now = Local::now();
    //                    let start = Instant::now();
    //                    job.restart().await;
    //                    job.to_wait().await;
    //                    let elapsed_time = start.elapsed();
    //                    if !quiet {
    //                        println!("----------------------------------");
    //                        println!(
    //                            "Started: {}",
    //                            now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    //                        );
    //                        println!("Ran: {}", exe_path.display());
    //                        println!("Took: {}ms", elapsed_time.as_millis(),);
    //                    }
    //                };
    //                action
    //            })
    //        });
    //        let watch_path = WatchedPath::non_recursive(file_path);
    //        wx.config.pathset(vec![watch_path]);
    //        let _ = wx.main().await?;
    //    } else {
    //        eprintln!("Error: file '{}' does not exist", file_path.display());
    //        std::process::exit(1);
    //    }
    //};

    Ok(())
}
