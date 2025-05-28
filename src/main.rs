use anyhow::Result;
use anyhow::anyhow;
use chrono::Local;
use clap::{arg, command};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use watchexec::WatchedPath;
use watchexec::Watchexec;
use watchexec::command::Command as WatchCommand;
use watchexec::command::Program;
use watchexec::command::Shell;
use watchexec_signals::Signal;

struct Runner {
    quiet: bool,
    raw_file_path: Option<PathBuf>,
    raw_then_path: Option<PathBuf>,
    requested_path: PathBuf,
}

impl Runner {
    pub fn cd_to(&self) -> Option<PathBuf> {
        match self.requested_path.parent() {
            Some(p) => Some(p.to_path_buf()),
            None => None,
        }
    }

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
    -q --quiet "Only print script output"))
            .arg(
                arg!(
    -t --then <then_path>
                "Script to run after the main process is done")
                .value_parser(clap::value_parser!(PathBuf)),
            )
            .get_matches();
        let requested_path = matches
            .get_one::<PathBuf>("file_path")
            .unwrap()
            .to_path_buf();
        if !requested_path.exists() {
            eprintln!("ERROR: {} does not exist", requested_path.display());
            std::process::exit(1);
        }
        let r = Runner {
            quiet: matches.get_flag("quiet"),
            raw_file_path: matches.get_one::<PathBuf>("file_path").cloned(),
            raw_then_path: matches.get_one::<PathBuf>("then").cloned(),
            // TODO: deprecate requested_path to use
            // method calls on raw_file_path
            requested_path,
        };
        r.validate_paths();
        r
    }

    pub async fn run(&self) -> Result<()> {
        let wx = Watchexec::default();
        let cd_to = self.cd_to();
        let quiet = self.quiet.clone();
        let requested_path = self.requested_path.clone();
        let script_name = self.script_name()?;
        let watch_command = self.watch_command();
        clearscreen::clear().unwrap();
        if !quiet {
            println!("Watching: {}", requested_path.display());
        }
        wx.config.on_action(move |mut action| {
            clearscreen::clear().unwrap();
            if let Some(target_dir) = cd_to.as_ref() {
                let _ = std::env::set_current_dir(target_dir).is_ok();
            }
            let cd_to = cd_to.clone();
            let quiet = quiet.clone();
            let script_name = script_name.clone();
            let watch_command = watch_command.clone();
            if action.signals().any(|sig| sig == Signal::Interrupt) {
                action.quit(); // Needed for Ctrl+c
            } else {
                action.list_jobs().for_each(|(_, job)| {
                    job.delete_now();
                });
                let (_, job) = action.create_job(watch_command.clone());
                let now = Local::now();
                let start = Instant::now();
                job.start();
                tokio::spawn(async move {
                    job.to_wait().await;
                    if !job.is_dead() {
                        let elapsed_time = start.elapsed();
                        if !quiet {
                            println!("-----------------------------------");
                            println!(
                                "started | {}",
                                now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                            );
                            if let Some(cded) = cd_to {
                                if cded != Path::new("") && cded != Path::new(".") {
                                    println!("cd      | {}", cded.display());
                                }
                            }
                            println!("ran     | {}", script_name);
                            println!("took    | {}ms", elapsed_time.as_millis(),);
                            println!("-----------------------------------");
                        }
                    }
                });
            }
            action
        });
        let watch_path = WatchedPath::non_recursive(&self.requested_path.to_path_buf());
        wx.config.pathset(vec![watch_path]);
        let _ = wx.main().await?;
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

    pub fn validate_paths(&self) {
        if let Some(file_path) = &self.raw_file_path {
            if !file_path.exists() {
                eprintln!("ERROR: {} does not exist", file_path.display());
                std::process::exit(1);
            }
        }
        if let Some(then_path) = &self.raw_then_path {
            if !then_path.exists() {
                eprintln!("ERROR: {} does not exist", then_path.display());
                std::process::exit(1);
            }
        }
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
    Ok(())
}
