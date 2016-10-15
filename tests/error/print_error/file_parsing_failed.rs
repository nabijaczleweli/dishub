use dishub::Error;
use std::iter::FromIterator;


#[test]
fn empty() {
    let mut out = Vec::new();
    Error::FileParsingFailed {
            desc: "leaderboard",
            errors: vec![],
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Failed to parse leaderboard.\n".to_string());
}

#[test]
fn not_empty() {
    let mut out = Vec::new();
    Error::FileParsingFailed {
            desc: "leaderboard",
            errors: vec!["leaderboard.toml: 12:36: unexpected value".to_string(), "leaderboard.toml: 15:11: too many values".to_string()],
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
               "Failed to parse leaderboard:\n\
                \x20\x20leaderboard.toml: 12:36: unexpected value\n\
                \x20\x20leaderboard.toml: 15:11: too many values\n"
                   .to_string());
}
