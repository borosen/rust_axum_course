use std::fmt::Display;
use std::str::FromStr;

use crate::config;
use crate::crypt::{Error, Result};
use crate::utils::{b64u_decode, b64u_encode};

// region:    --- Token Type

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
            exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
            sign_b64u: sign_b64u.to_owned(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();

    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = &config();

    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Token Gen and Validation

fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    todo!()
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    todo!()
}

fn _token_sign_into_b64u(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<String> {
    todo!()
}

// endregion: --- (private) Token Gen and Validation

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnRfMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64-encoded";
        let fx_token = Token {
            ident: "fx-ident_01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64-encoded".to_string(),
        };

        assert_eq!(fx_token_str, fx_token.to_string());

        Ok(())
    }

    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnRfMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64-encoded";
        let fx_token = Token {
            ident: "fx-ident_01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64-encoded".to_string(),
        };

        let token: Token = fx_token_str.parse()?;

        assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));

        Ok(())
    }
}
