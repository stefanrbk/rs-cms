use core::cmp::Ordering;
use std::fmt::Display;

use regex::Regex;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SemVer(u32, u32, u32);

impl SemVer {
    pub const fn new(major:u32, minor:u32, patch:u32) -> Self {
        SemVer(major, minor, patch)
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}
