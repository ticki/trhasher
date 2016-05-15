use std::hash::{BuildHasher, Hasher};
use std::{time, fmt};

pub struct Report {
    byte_dist: u32,
    byte_filled: u8,
    bit_fair: bool,
    buckets_max_collisions: u32,
    buckets_filled: u32,
    buckets_dist: u32,
    average: f64,
    and_zero: u32,
    // runs: u32,
    // modulo: u32,
    time: time::Duration,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "bytes χ²:           {}", self.byte_dist)?;
        writeln!(f, "bytes completeness: {}", self.byte_filled)?;
        writeln!(f, "bit fairness:       {}", self.bit_fair)?;
        writeln!(f, "maximal bucket:     {}", self.buckets_max_collisions)?;
        writeln!(f, "buckets filled:     {}", self.buckets_filled)?;
        writeln!(f, "buckets χ²:         {}", self.buckets_dist)?;
        writeln!(f, "average:            {}", self.average)?;
        writeln!(f, "AND zero:           {}", self.and_zero)?;
        write!  (f, "time:               {:3>} s. {:9>} ns.", self.time.as_secs(), self.time.subsec_nanos())?;
    }
}

impl Report {
    pub fn new<I: Iterator<Item = u64> + Clone>(iter: I) -> Report {
        let time = time::Instant::now();

        let byte_dist;
        let byte_filled;

        {
            let (rounds, bytes) = bytes(iter.clone());
            let expected = rounds / 256;

            byte_dist = bytes.iter().map(|&x| ((x as i32 - expected as i32) * (x as i32 - expected as i32)) as u32).sum();
            byte_filled = bytes.iter().map(|&x| (x >= 1) as u8).sum();
        }

        let buckets_dist;
        let buckets_filled;
        let buckets_max_collisions;

        {
            let (rounds, buckets) = buckets(iter.clone());
            let expected = rounds / 4096;

            buckets_dist = buckets.iter().map(|&x| ((x as i32 - expected as i32) * (x as i32 - expected as i32)) as u32).sum();
            buckets_filled = buckets.iter().map(|&x| (x >= 1) as u32).sum();
            buckets_max_collisions = buckets.into_iter().max().map_or(0, |&x| x);
        }

        Report {
            byte_dist: byte_dist,
            byte_filled: byte_filled,
            bit_fair: bit_fair(iter.clone()),
            buckets_max_collisions: buckets_max_collisions,
            buckets_filled: buckets_filled,
            buckets_dist: buckets_dist,
            and_zero: and_zero(iter.clone()),
            average: {
                let (sum, n) = iter.fold((0.0, 0u32), |(sum, n), h| (sum + h as f64 / !0u64 as f64, n + 1));
                sum / n as f64
            },
            time: time.elapsed(),
        }
    }
}

fn bytes<I: Iterator<Item = u64>>(iter: I) -> (u32, [u32; 256]) {
    let mut bytes = [0; 256];
    let mut rounds = 0;

    for i in iter.take(1000000) {
        bytes[(i as usize & 255) as usize] += 1;
        bytes[(i >> 8 as usize & 255) as usize] += 1;
        bytes[(i >> 16 as usize & 255) as usize] += 1;
        bytes[(i >> 24 as usize & 255) as usize] += 1;
        bytes[(i >> 32 as usize & 255) as usize] += 1;
        bytes[(i >> 40 as usize & 255) as usize] += 1;
        bytes[(i >> 48 as usize & 255) as usize] += 1;
        bytes[(i >> 56 as usize & 255) as usize] += 1;

        rounds += 8;
    }

    (rounds, bytes)
}

fn bit_fair<I: Iterator<Item = u64>>(iter: I) -> bool {
    iter.take(1000000).map(|x| {
        let h = x.count_ones();
        ((h as i64 - 32 as i64) * (h as i64 - 32 as i64)) as u64
    }).sum::<u64>() < 5
}

fn and_zero<I: Iterator<Item = u64>>(iter: I) -> u32 {
    let mut cur = !0;
    let mut rounds = 0;

    for i in iter.take(1000000) {
        cur &= i;
        rounds += 1;

        if cur == 0 {
            break;
        }
    }

    rounds
}

fn buckets<I: Iterator<Item = u64>>(iter: I) -> (u32, [u32; 4096]) {
    let mut buckets = [0; 4096];
    let mut rounds = 0;

    for i in iter.take(1000000) {
        buckets[i as usize % 4096] += 1;

        rounds += 1;
    }

    (rounds, buckets)
}
