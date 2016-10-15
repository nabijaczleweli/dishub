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
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
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
        }
    }
}
