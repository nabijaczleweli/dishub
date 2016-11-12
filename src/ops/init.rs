//! This module contains the functions used only by the `init` subsystem.
//!
//! The flow of the `init` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::init::verify()
//! |> ops::init::get_data()
//! |> ops::AppTokens::write()
//! ```


use self::super::super::util::prompt_exact_len;
use self::super::{AppTokens, verify_file};
use self::super::super::Error;
use std::io::{BufRead, Write};
use std::path::PathBuf;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `init` subsystem.
///
/// The return value contains either the path to the file containing the global app tokens or why getting it failed.
///
/// # Examples
///
/// Verifying a nonexistant file or an existing file with forcing.
///
/// ```
/// # use std::env::temp_dir;
/// # use dishub::ops::init;
/// let tf = temp_dir().join("dishub-doctest").join("ops-init-verify-0");
/// assert_eq!(init::verify(&("$TEMP/ops-init-verify-0".to_string(), tf.clone()), true),
///            Ok(tf.join("tokens.toml")));
/// ```
///
/// Verifying an existing file without forcing.
///
/// ```
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use dishub::ops::init;
/// # use dishub::Error;
/// # use std::io::Write;
/// let tf = temp_dir().join("dishub-doctest").join("ops-init-verify-1");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("tokens.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(init::verify(&("$TEMP/ops-init-verify-1".to_string(), tf), false),
///            Err(Error::OverrideNoForce("$TEMP/ops-init-verify-1/tokens.toml".to_string())));
/// ```
pub fn verify(config_dir: &(String, PathBuf), force: bool) -> Result<PathBuf, Error> {
    verify_file("tokens.toml", false, config_dir, force, "")
}

/// Prompt the user for application data.
///
/// # Examples
///
/// ```
/// # use dishub::ops::{init, AppTokens};
/// # use std::io::BufReader;
/// assert_eq!(init::get_data(
///     &mut BufReader::new(&b"994c365aec1700f5783bac697e2347ffd2268e1e\n\
///                            a8233f2465e4e27b36c3e9a9.5ec9ab.e8fe135112b5f4d678bd1d221f8\n"[..]),
///     &mut Vec::new()),
///     AppTokens {
///         github: "994c365aec1700f5783bac697e2347ffd2268e1e".to_string(),
///         discord: "a8233f2465e4e27b36c3e9a9.5ec9ab.e8fe135112b5f4d678bd1d221f8".to_string(),
///     });
/// ```
pub fn get_data<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> AppTokens {
    AppTokens {
        github: prompt_exact_len(input, output, "GitHub OAuth token", |_| true, 40).unwrap(),
        discord: prompt_exact_len(input, output, "Discord bot token", |_| true, 59).unwrap(),
    }
}
