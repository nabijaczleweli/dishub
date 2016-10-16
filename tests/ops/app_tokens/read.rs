use self::super::super::make_dir;
use dishub::ops::AppTokens;
use std::io::Write;
use std::fs::File;
use dishub::Error;


#[test]
fn correct() {
    let mut path = make_dir("app-tokens-read", "correct");
    path.push("tokens.toml");

    let _ = File::create(&path).unwrap().write_all(br#"
        discord = "EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob"
        github = "YO4QXSAMh72MivCCOYpRWxNx0ZpEwocF8DM1D130"
    "#);

    assert_eq!(AppTokens::read(&path),
               Ok(AppTokens {
                   discord: "EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob".to_string(),
                   github: "YO4QXSAMh72MivCCOYpRWxNx0ZpEwocF8DM1D130".to_string(),
               }));
}

#[test]
fn incorrect() {
    let mut path = make_dir("app-tokens-read", "incorrect");
    path.push("tokens.toml");

    let _ = File::create(&path).unwrap().write_all(br#"
        discord = "EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob"
    "#);

    assert_eq!(AppTokens::read(&path),
               Err(Error::FileParsingFailed {
                   desc: "App tokens",
                   errors: vec![],
               }));
}

#[test]
fn nonexistant() {
    assert_eq!(AppTokens::read(&make_dir("app-tokens-read", "nonexistant")),
               Err(Error::Io {
                   desc: "App tokens",
                   op: "open",
               }));
}
