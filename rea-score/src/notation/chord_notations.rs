use std::{error::Error, str::FromStr};

use super::{get_token, reascore_tokens, NotationError};

#[derive(Debug, PartialEq)]
pub enum ChordNotations {
    Dynamics(String),
}
impl ToString for ChordNotations {
    fn to_string(&self) -> String {
        match self {
            Self::Dynamics(idx) => format!("dyn:{}", idx),
        }
    }
}
impl FromStr for ChordNotations {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = reascore_tokens(s, None)?;
        match tokens[0] {
            "dyn" => {
                let expr = get_token(&tokens, 1)?;
                Ok(Self::Dynamics(expr.to_string()))
            }
            x => Err(NotationError::UnexpectedToken(x.to_string()).into()),
        }
    }
}