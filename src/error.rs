use std::{error::Error, fmt};

use crate::Token;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct UnexpectedToken {
    pub token: Token,
    pub position: usize,
}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected token {:?}\nPosition: {}",
            self.token, self.position
        )
    }
}

impl Error for UnexpectedToken {}
