use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl XorShift64Star { fn new () -> Self { let mut seed = 0 ; while seed == 0 { let mut hasher = DefaultHasher :: new () ; static COUNTER : AtomicUsize = AtomicUsize :: new (0) ; hasher . write_usize (COUNTER . fetch_add (1 , Ordering :: Relaxed)) ; seed = hasher . finish () ; } XorShift64Star { state : Cell :: new (seed) } } fn next (& self) -> u64 { let mut x = self . state . get () ; debug_assert_ne ! (x , 0) ; x ^= x >> 12 ; x ^= x << 25 ; x ^= x >> 27 ; self . state . set (x) ; x . wrapping_mul (0x2545_f491_4f6c_dd1d) } # [doc = " Return a value from `0..n`."] fn next_usize (& self , n : usize) -> usize { (self . next () % n as u64) as usize } }
}