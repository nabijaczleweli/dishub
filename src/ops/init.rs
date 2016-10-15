use self::super::super::util::prompt_exact_len;
use self::super::{AppTokens, verify_file};
use self::super::super::Error;
use std::io::{BufRead, Write};
use std::path::PathBuf;


pub fn verify(config_dir: &(String, PathBuf), force: bool) -> Result<PathBuf, Error> {
    verify_file("tokens.toml", false, config_dir, force, "")
}

pub fn get_data<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> AppTokens {
    AppTokens {
        github: prompt_exact_len(input, output, "GitHub OAuth token", |_| true, 40).unwrap(),
        discord: prompt_exact_len(input, output, "Discord bot token", |_| true, 59).unwrap(),
    }
}
