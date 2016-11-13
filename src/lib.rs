//! [dishub](https://github.com/nabijaczleweli/dishub) is an app for posting GitHub activity on Discord.
//!
//! # Library doc
//!
//! This library is used by `dishub` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! See documentation for `ops::*` submodules as each one has a distinct data flow.
//!
//! # Executable doc
//!
//! Exit values and possible errors:
//!
//! ```plaintext
//! 1 - File would need to be overriden but `-f` not specified
//! 2 - File from a subsystem doesn't exist
//! 3 - Couldn't parse a file
//! 4 - An I/O error au general
//! 5 - Watched subject does not exist
//! 6 - Failed to log in to a service
//! ```
//!
//! ## Executable manpage
//!
//! [All manpages](https://cdn.rawgit.com/nabijaczleweli/dishub/man/index.html)


extern crate rustc_serialize;
#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate hyper;
extern crate json;
#[macro_use]
extern crate clap;
extern crate toml;


mod error;

pub mod ops;
pub mod util;
pub mod options;

pub use error::Error;
