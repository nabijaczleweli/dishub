//! Main functions doing actual work.
//!
//! Each module contains the functions for their respective subsystems.


use toml::{Parser, Value, decode};
use rustc_serialize::Decodable;
use std::path::{PathBuf, Path};
use self::super::Error;
use std::fs::File;
use std::io::Read;

mod app_tokens;

pub mod init;

pub use self::app_tokens::AppTokens;


fn verify_file(fname: &str, should_exist: bool, config_dir: &(String, PathBuf), force: bool, producing_subsystem: &'static str) -> Result<PathBuf, Error> {
    let app_data_file = config_dir.1.join(fname);

    if force || app_data_file.exists() == should_exist {
        Ok(app_data_file)
    } else {
        let filename = PathBuf::from(&config_dir.0).join(fname).to_str().unwrap().replace("\\", "/");

        if should_exist {
            Err(Error::RequiredFileFromSubsystemNonexistant {
                subsys: producing_subsystem,
                fname: filename,
            })
        } else {
            Err(Error::OverrideNoForce(filename))
        }
    }
}

fn read_toml_file<T: Decodable>(p: &Path, desc: &'static str) -> Result<T, Error> {
    let mut buf = String::new();
    try!(File::open(p).map_err(|_| "open").and_then(|mut p| p.read_to_string(&mut buf).map_err(|_| "read")).map_err(|op| {
        Error::Io {
            desc: desc,
            op: op,
        }
    }));

    let mut parser = Parser::new(&buf);
    let parsed = parser.parse().and_then(|t| decode(Value::Table(t)));
    parsed.ok_or_else(move || {
        Error::FileParsingFailed {
            desc: desc,
            errors: parser.errors
                .iter()
                .map(|e| {
                    let (line, col) = parser.to_linecol(e.lo);
                    format!("error: {}:{}: {}", line, col, e.desc)
                })
                .collect(),
        }
    })
}
