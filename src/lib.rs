use serde::{Serialize, Deserialize};
use core::fmt;

pub type Edge = (i32, i32);

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add(Edge),
    Remove(Edge),
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out;
        match *self {
            Msg::Add(edge) => {
                out = format!("Add {} {}", edge.0, edge.1);
            }
            Msg::Remove(edge) => {
                out = format!("Remove {} {}", edge.0, edge.1);
            }
        }
        write!(f, "{}", out)
    }
}

pub mod algorithms;
