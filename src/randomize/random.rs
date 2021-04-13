use rand::Rng;
use std::ops::Range;

pub fn make_random(range: Range<i64>) -> i64 {
    rand::thread_rng().gen_range(range)
}