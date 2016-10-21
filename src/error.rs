use self::super::util::uppercase_first;
use std::io::Write;


/// Enum representing all possible ways the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// The specified file would need to be overriden but was not allowed to.
    OverrideNoForce(String),
    /// The specified subsystem needs to be run beforehand to produce the specified file.
    RequiredFileFromSubsystemNonexistant {
        /// The subsystem that needs to be run.
        subsys: &'static str,
        /// The file the specified subsystem produces.
        fname: String,
    },
    /// Failed to parse the specified file because of the specified errors.
    FileParsingFailed {
        /// The file that failed to parse.
        desc: &'static str,
        /// The parsing errors that occured.
        errors: Vec<String>,
    },
    /// An I/O error occured.
    ///
    /// This includes higher-level I/O errors like FS ones.
    Io {
        /// The file the I/O operation regards.
        desc: &'static str,
        /// The failed operation.
        ///
        /// This should be lowercase and imperative ("create", "open").
        op: &'static str,
    },
    /// A watched item does not exist.
    WatchedDoesNotExist {
        /// The type of nonexistant resource.
        tp: &'static str,
        /// The name of the nonexistant resource.
        name: String,
    },
    /// Failed to log in to the specified service.
    LoginFailed(&'static str),
}

impl Error {
    /// Write the error message to the specified output stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dishub::Error;
    /// # use std::iter::FromIterator;
    /// let mut out = Vec::new();
    /// Error::FileParsingFailed {
    ///     desc: "leaderboard",
    ///     errors: vec![],
    /// }.print_error(&mut out);
    /// assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
    ///            "Failed to parse leaderboard.\n".to_string());
    /// ```
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Error::OverrideNoForce(ref fname) => {
                writeln!(err_out, "File \"{}\" was not overriden to prevent data loss.", fname).unwrap();
                writeln!(err_out, "Pass --force to override it.").unwrap();
            }
            Error::RequiredFileFromSubsystemNonexistant { subsys, ref fname } => {
                writeln!(err_out, "Run the {} subsystem first to produce \"{}\".", subsys, fname).unwrap()
            }
            Error::FileParsingFailed { desc, ref errors } => {
                writeln!(err_out, "Failed to parse {}{}", desc, if errors.is_empty() { '.' } else { ':' }).unwrap();
                for err in errors {
                    writeln!(err_out, "  {}", err).unwrap()
                }
            }
            Error::Io { desc, op } => {
                // Strip the last 'e', if any, so we get correct inflection for continuous times
                let op = uppercase_first(if op.ends_with('e') {
                    &op[..op.len() - 1]
                } else {
                    op
                });
                writeln!(err_out, "{}ing {} failed.", op, desc).unwrap()
            }
            Error::WatchedDoesNotExist { tp, ref name } => writeln!(err_out, "The watched {} \"{}\" doesn't exist.", tp, name).unwrap(),
            Error::LoginFailed(service) => writeln!(err_out, "Failed to log in to {}.", service).unwrap(),
        }
    }

    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dishub::Error;
    /// assert_eq!(Error::FileParsingFailed {
    ///     desc: "",
    ///     errors: vec![],
    /// }.exit_value(), 3);
    /// ```
    pub fn exit_value(&self) -> i32 {
        match *self {
            Error::OverrideNoForce(_) => 1,
            Error::RequiredFileFromSubsystemNonexistant { .. } => 2,
            Error::FileParsingFailed { .. } => 3,
            Error::Io { .. } => 4,
            Error::WatchedDoesNotExist { .. } => 5,
            Error::LoginFailed(_) => 6,
        }
    }
}
