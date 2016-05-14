use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};

use itertools::Itertools;

use analysis;

pub struct Transforms {
    plain: analysis::Report,
    xor: analysis::Report,
    add: analysis::Report,
    prime_multiply: analysis::Report,
    double: analysis::Report,
    had: analysis::Report,
    skip: analysis::Report,
}

impl fmt::Display for Transforms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "~ plain")?;
        writeln!(f, "{}", self.plain)?;
        writeln!(f, "~ XOR fold")?;
        writeln!(f, "{}", self.xor)?;
        writeln!(f, "~ add fold")?;
        writeln!(f, "{}", self.add)?;
        writeln!(f, "~ multiply by prime")?;
        writeln!(f, "{}", self.prime_multiply)?;
        writeln!(f, "~ double hash")?;
        writeln!(f, "{}", self.double)?;
        writeln!(f, "~ Hadamard transform")?;
        writeln!(f, "{}", self.had)?;
        writeln!(f, "~ skip every two")?;
        writeln!(f, "{}", self.skip)?;
    }
}

impl Transforms {
    pub fn new<B, I, T, H>(builder: B, iter: I) -> Transforms
        where B: BuildHasher<Hasher = H>,
              I: Iterator<Item = T> + Clone,
              T: Hash,
              H: Clone + Hasher {
        let hasher = builder.build_hasher();
        let hash = iter.map(|x| {
            let mut hasher = hasher.clone();
            x.hash(&mut hasher);
            hasher.finish()
        });

        Transforms {
            plain: analysis::Report::new(hash),
            xor: analysis::Report::new(StateMap {
                iter: hash,
                fun: |state, x| (state ^ x, state ^ x),
                state: 0,
            }),
            add: analysis::Report::new(StateMap {
                iter: hash,
                fun: |state: u64, x| (state.wrapping_add(x), state.wrapping_add(x)),
                state: 0,
            }),
            prime_multiply: analysis::Report::new(hash.map(|x| x.wrapping_mul(0x1FFFFFFFFFFFFFFF))),
            double: analysis::Report::new(hash.map(|x| {
                let mut hasher = hasher.clone();
                x.hash(&mut hasher);
                hasher.finish()
            })),
            had: analysis::Report::new(hash.map(|x| had(x))),
            skip: analysis::Report::new(hash.step(2)),
        }
    }
}

/// Hadamard-like transform.
fn had(mut x: u64) -> u64 {
    x ^= x >> 32 & 0x00000000FFFFFFFF;
    x ^= x << 32 & 0xFFFFFFFF00000000;
    x ^= x >> 16 & 0x0000FFFF0000FFFF;
    x ^= x << 16 & 0xFFFF0000FFFF0000;
    x ^= x >> 8  & 0x00FF00FF00FF00FF;
    x ^= x << 8  & 0xFF00FF00FF00FF00;
    x ^= x >> 4  & 0x0F0F0F0F0F0F0F0F;
    x ^= x << 4  & 0xF0F0F0F0F0F0F0F0;
    x ^= x >> 2  & 0x3333333333333333;
    x ^= x << 2  & 0xCCCCCCCCCCCCCCCC;
    x ^= x >> 1  & 0xAAAAAAAAAAAAAAAA;
    x ^= x << 1  & 0x5555555555555555;
    x
}

pub struct StateMap<I, F, S> {
    iter: I,
    fun: F,
    state: S,
}

impl<I, F, S, T, U> Iterator for StateMap<I, F, S>
    where F: Fn(S, T) -> (S, U),
          I: Iterator<Item = T> {
    type Item = U;

    fn next(&mut self) -> Option<U> {
        self.iter.next().map(|x| {
            let (state, res) = (self.fun)(self.state, x);
            self.state = state;
            res
        })
    }
}
