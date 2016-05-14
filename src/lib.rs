#![feature(test, question_mark)]

use std::fmt;

extern crate test;
extern crate rand;
extern crate itertools;

mod analysis;
mod inspect;
mod bad_rand;
mod stream;
mod data;

struct Suit {
    hashes: Vec<(&'static str, inspect::Test)>,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (a, b) in self.hashes {
            writeln!(f, "--- {} ---", a)?;
            writeln!(f, "{}", b)?;
        }

        Ok(())
    }
}
