use dishub::ops::{init, AppTokens};
use std::iter::FromIterator;
use std::io::Cursor;

mod verify;


#[test]
fn get_data() {
    let mut out = Vec::new();
    let result = init::get_data(&mut Cursor::new(&b"YO4QXSAMh72MivCCOYpRWxNx0ZpEwocF8DM1D130\n\
                                                    EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob\n"[..]),
                                &mut out);

    assert_eq!(result,
               AppTokens {
                   discord: "EaCdWByA0BTPnlt6OYpnuAc0.cfPzzG.7UuaJIiPatD507FYjgBcHw3Ecob".to_string(),
                   github: "YO4QXSAMh72MivCCOYpRWxNx0ZpEwocF8DM1D130".to_string(),
               });
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
               "GitHub OAuth token: Discord bot token: ".to_string());
}
