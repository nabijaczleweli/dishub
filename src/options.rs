//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use dishub::options::Options;
//! let options = Options::parse();
//! println!("Config directory: {}", options.config_dir.0);
//! ```


use clap::{self, App, SubCommand, Arg, AppSettings};
use std::path::PathBuf;
use std::env::home_dir;
use std::fs;


/// All possible subsystems, think `cargo`'s or `git`'s.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Subsystem {
    /// Initialise global app data
    Init {
        /// Whether to override current app configuration. Default: `false`
        force: bool,
    },
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory containing configuration. Default: `"$HOME/.dishub"`
    pub config_dir: (String, PathBuf),
    /// The specified subsystem.
    pub subsystem: Subsystem,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("dishub")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::VersionlessSubcommands)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Rust app for posting GitHub activity on Discord")
            .arg(Arg::from_usage("-c --config-dir=[CONFIG_DIR] 'Directory containing configuration. Default: $HOME/.dishub'")
                .validator(Options::config_dir_validator))
            .subcommand(SubCommand::with_name("init")
                .about("Initialise global app data")
                .arg(Arg::from_usage("-f --force 'Override current app configuration'")))
            .get_matches();

        Options {
            config_dir: match matches.value_of("config-dir") {
                Some(dirs) => (dirs.to_string(), fs::canonicalize(dirs).unwrap()),
                None => {
                    match home_dir() {
                        Some(mut hd) => {
                            hd = hd.canonicalize().unwrap();
                            hd.push(".dishub");

                            fs::create_dir_all(&hd).unwrap();
                            ("$HOME/.dishub".to_string(), hd)
                        }
                        None => {
                            clap::Error {
                                    message: "Couldn't automatically get home directory, please specify configuration directory with the -c option".to_string(),
                                    kind: clap::ErrorKind::MissingRequiredArgument,
                                    info: None,
                                }
                                .exit()
                        }
                    }
                }
            },
            subsystem: match matches.subcommand() {
                ("init", Some(init_matches)) => Subsystem::Init { force: init_matches.is_present("force") },
                _ => panic!("No subcommand passed"),
            },
        }
    }

    fn config_dir_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Configuration directory \"{}\" not found", s))
    }
}
