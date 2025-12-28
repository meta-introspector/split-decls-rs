use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " [xorshift*] is a fast pseudorandom number generator which will"] # [doc = " even tolerate weak seeding, as long as it's not zero."] # [doc = ""] # [doc = " [xorshift*]: https://en.wikipedia.org/wiki/Xorshift#xorshift*"] struct XorShift64Star { state : Cell < u64 > , }