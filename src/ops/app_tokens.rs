use self::super::read_toml_file;
use self::super::super::Error;
use hubcaps::Credentials;
use toml::encode_str;
use std::path::Path;
use std::io::Write;
use std::fs::File;


/// The tokens needed to authenticate the app to GitHub.
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct AppTokens {
    /// GitHub OAuth authentication token
    ///
    /// Required scopes: public_repo, repo:status.
    pub github: String,
    /// Discord bot user token
    pub discord: String,
}

impl AppTokens {
    /// Read the application GitHub tokens from the specified file.
    pub fn read(p: &Path) -> Result<AppTokens, Error> {
        read_toml_file(p, "App tokens")
    }

    /// Save the application GitHub tokens to the specified file.
    pub fn write(&self, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&self).as_bytes()).unwrap();
    }

    /// Get the GitHub credentials directly pluggable into `hubcaps`.
    pub fn github_credentials(&self) -> Credentials {
        Credentials::Token(self.github.clone())
    }
}
