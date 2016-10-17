use dishub::ops::AppTokens;
use self::super::make_dir;
use std::io::Read;
use std::fs::File;

mod read;


#[test]
fn write() {
    let mut path = make_dir("app-tokens", "write");
    path.push("tokens.toml");

    AppTokens {
            discord: "EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob".to_string(),
            github: "YO4QXSAMh72MivCCOYpRWxNx0ZpEwocF8DM1D130".to_string(),
        }
        .write(&path);

    let mut buf = String::new();
    let _ = File::open(&path)
        .unwrap()
        .read_to_string(&mut buf);

    assert_eq!(&buf,
               "discord = \"EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob\"\n\
                github = \"YO4QXSAMh72MivCCOYpRWxNx0ZpEwocF8DM1D130\"\n");
}
