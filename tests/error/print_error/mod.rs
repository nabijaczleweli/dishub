use dishub::Error;
use std::iter::FromIterator;

mod file_parsing_failed;


#[test]
fn override_no_force() {
    let mut out = Vec::new();
    Error::OverrideNoForce("$HOME/.dishub/app.toml".to_string()).print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
               "File \"$HOME/.dishub/app.toml\" was not overriden to prevent data loss.\n\
                Pass --force to override it.\n"
                   .to_string());
}

#[test]
fn required_file_from_subsystem_nonexistant() {
    let mut out = Vec::new();
    Error::RequiredFileFromSubsystemNonexistant {
            subsys: "init",
            fname: "$HOME/.dishub/app.toml".to_string(),
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
               "Run the init subsystem first to produce \"$HOME/.dishub/app.toml\".\n".to_string());
}
