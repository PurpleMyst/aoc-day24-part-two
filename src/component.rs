use std::{fmt, num::ParseIntError, str::FromStr};

use super::rpds;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Component {
    pub left: u8,
    pub right: u8,
}

impl FromStr for Component {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();

        Ok(Self {
            left: parts[0].parse()?,
            right: parts[1].parse()?,
        })
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.left, self.right)
    }
}

pub type Components = rpds::HashTrieSet<Component>;
