use std::fmt::Display;

use serde::{Deserialize, Serialize};

use colored::Colorize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Part {
    A(u64),
    B(u64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub part: Part,
    pub year: u8,
    pub day: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Complete(Answer),
    Verified(Answer),
    Incorrect(Answer),
    Empty(u8, u8),
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Complete(a) => {
                let (part, answer) = match a.part {
                    Part::A(a) => ('a', a.to_string()),
                    Part::B(b) => ('b', b.to_string()),
                };
                write!(
                    f,
                    "{} {}: {}",
                    "✓".to_string().blue(),
                    format!("[{} | {} | {part}]", a.year, a.day).bright_black(),
                    format!("{answer}").underline(),
                )
            }
            State::Verified(a) => {
                let (part, answer) = match a.part {
                    Part::A(a) => ('a', a.to_string()),
                    Part::B(b) => ('b', b.to_string()),
                };
                write!(
                    f,
                    "{} {}: {}",
                    "☑".to_string().blue(),
                    format!("[{} | {} | {part}]", a.year, a.day).bright_black(),
                    format!("{answer}").underline(),
                )
            }
            State::Incorrect(a) => {
                let (part, answer) = match a.part {
                    Part::A(a) => ('a', a.to_string()),
                    Part::B(b) => ('b', b.to_string()),
                };
                write!(
                    f,
                    "{} {}: {}",
                    "X".to_string().blue(),
                    format!("[{} | {} | {part}]", a.year, a.day).bright_black(),
                    format!("{answer}").underline(),
                )
            }
            State::Empty(year, day) => {
                write!(
                    f,
                    "{} {}: {}",
                    "-".to_string().bright_black(),
                    format!("[{} | {} | _]", year, day).bright_black(),
                    format!(""),
                )
            }
        }
    }
}
