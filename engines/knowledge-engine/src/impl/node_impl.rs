use core::fmt;
use std::str::FromStr;

use crate::objects::node::Node;

// this is not the final version, it's going to be serialized to a binary format, so this is just temporary overhead.
impl fmt::Display for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}