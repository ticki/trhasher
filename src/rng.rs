use std::marker::PhantomData;

use rand;

#[derive(Clone, Copy)]
pub struct RandIter<T> {
    _phantom: PhantomData<T>,
}

impl<T> RandIter<T> {
    pub fn new() -> RandIter<T> {
        RandIter {
            _phantom: PhantomData,
        }
    }
}

impl<T: rand::Rand> Iterator for RandIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        rand::random()
    }
}

#[derive(Copy, Clone)]
pub struct BadRand {
    state: u64,
}

impl BadRand {
    pub fn new() -> BadRand {
        BadRand {
            state: 0xABFABFABF1015,
        }
    }
}

impl Iterator for BadRand {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // Wow, full cycle! High quality randomness, huh?
        self.state += 1;
        Some((self.state ^ 0xD00DD00DD000DD0F).rotate_left(7).wrapping_add(0x2947ABCD))
    }
}
