#![feature(test, question_mark)]

use std::fmt;
use std::hash::BuildHasher;

extern crate test;
extern crate rand;
extern crate itertools;

mod analysis;
mod data;
mod inspect;
mod rng;
mod stream;

pub struct Suit {
    hashes: Vec<(&'static str, inspect::Test)>,
}

impl Suit {
    pub fn new() -> Suit {
        Suit {
            hashes: Vec::new(),
        }
    }

    pub fn add<B>(&mut self, name: &'static str, build: B)
        where B: BuildHasher,
              B::Hasher: Clone {
        self.hashes.push((name, inspect::Test::test(build)));
    }
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
