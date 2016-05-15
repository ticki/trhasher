use std::hash::{Hasher, BuildHasher};
use std::{time, iter, fmt};

use {rng, data, test, stream, analysis};

pub struct Test {
    dict: stream::Transforms,
    rand: stream::Transforms,
    ascii: stream::Transforms,
    bad_rand: stream::Transforms,
    num: stream::Transforms,
    primes: stream::Transforms,
    rehash: analysis::Report,
    zero_sensitive: bool,
    deterministic: bool,
    gb_per_sec: f64,
    total_time: time::Duration,
    points: u64,
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "- Stream quality")?;
        writeln!(f, "[[English dictionary]]")?;
        writeln!(f, "{}", self.dict)?;
        writeln!(f, "[[Random numbers]]")?;
        writeln!(f, "{}", self.rand)?;
        writeln!(f, "[[Random ASCII]]")?;
        writeln!(f, "{}", self.ascii)?;
        writeln!(f, "[[Poor random]]")?;
        writeln!(f, "{}", self.bad_rand)?;
        writeln!(f, "[[Integers]]")?;
        writeln!(f, "{}", self.num)?;
        writeln!(f, "[[Prime numbers]]")?;
        writeln!(f, "{}", self.primes)?;
        writeln!(f, "- Rehashing quality")?;
        writeln!(f, "{}", self.rehash)?;
        writeln!(f, "- Properties")?;
        writeln!(f, "zero sensitive:     {}", self.zero_sensitive)?;
        writeln!(f, "deterministic:      {}", self.deterministic)?;
        writeln!(f, "- Performance")?;
        writeln!(f, "GB/s:               {}", self.gb_per_sec)?;
        write!  (f, "total time:         {:3>} s. {:9>} ns.", self.total_time.as_secs(), self.total_time.subsec_nanos())?;
        writeln!(f, "- Final result")?;
        write!  (f, "points:             {}", self.points)?;
    }
}

impl Test {
    pub fn test<B>(builder: B) -> Test
        where B: BuildHasher,
              B::Hasher: Clone {
        let time = time::Instant::now();
        let h = builder.build_hasher();

        Test {
            dict: stream::Transforms::new(builder, data::DICT.lines()),
            rand: stream::Transforms::new(builder, rng::RandIter::<u64>::new()),
            ascii: stream::Transforms::new(builder, rng::RandIter::<char>::new()),
            bad_rand: stream::Transforms::new(builder, rng::BadRand::new()),
            num: stream::Transforms::new(builder, 0..),
            primes: stream::Transforms::new(builder, data::PRIMES.iter()),
            rehash: analysis::Report::new(Rehasher {
                hasher: h.clone(),
                state: 0,
            }),
            zero_sensitive: {
                ({
                    let mut h = h.clone();
                    h.write_u8(0xFAB001005);
                    h.finish()
                } != {
                    let mut h = h.clone();
                    h.write_u8(0xFAB001005);
                    h.write_u8(0);
                    h.finish()
                }) && {
                    let mut h = h.clone();
                    h.write_u8(0xFAB001005);
                    h.write_u8(0);
                    h.finish()
                } != {
                    let mut h = h.clone();
                    h.write_u8(0xFAB001005);
                    h.write_u8(0);
                    h.write_u8(0);
                    h.finish()
                } && {
                    let mut h = h.clone();
                    h.write_u8(0);
                    h.finish()
                } != 0
            },
            deterministic: {
                let h0 = builder.build_hasher();
                let h1 = h0.clone();
                let h2 = h1.clone();
                let h3 = h2.clone();
                let h4 = h3.clone();

                h0.write(b"hello unicorn");
                h1.write(b"hello unicorn");
                h2.write(b"hello unicorn");
                h3.write(b"hello unicorn");
                h4.write(b"hello unicorn");

                let a = h0.finish();
                let b = h1.finish();
                let c = h2.finish();
                let d = h3.finish();
                let e = h4.finish();

                a == b && b == c && c == d && d == e
            },
            gb_per_sec: {
                let time = time::Instant::now();
                let mut hasher = builder.build_hasher();
                for i in 0..1000000000 / 8 {
                    hasher.write_u64(i);
                }
                test::black_box(hasher.finish());

                let elapsed = time.elapsed();

                64.0 / (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0)
            },
            total_time: time.elapsed(),
            points: 0, // TODO
        }
    }
}

#[derive(Clone)]
struct Rehasher<H: Clone> {
    hasher: H,
    state: u64,
}

impl<H: Hasher + Clone> Iterator for Rehasher<H> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let h = self.hasher.clone();
        h.write_u64(self.state);
        self.state = h.finish();

        Some(self.state)
    }
}
