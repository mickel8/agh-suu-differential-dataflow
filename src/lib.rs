use serde::{Serialize, Deserialize};
use core::fmt;

pub type Edge = (i32, i32);
pub type Time = i32;

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add(Edge, Time),
    Remove(Edge, Time),
    Result(Time)
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out;
        match *self {
            Msg::Add(edge, time) => {
                out = format!("Add {} {} time: {}", edge.0, edge.1, time);
            }
            Msg::Remove(edge, time) => {
                out = format!("Remove {} {} time: {}", edge.0, edge.1, time);
            }
            Msg::Result(time) => {
                out = format!("Result time: {}", time);
            }
        }
        write!(f, "{}", out)
    }
}

pub mod algorithms;
