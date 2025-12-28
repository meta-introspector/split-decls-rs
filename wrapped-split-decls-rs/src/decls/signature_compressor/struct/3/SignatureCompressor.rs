use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Signature compression system using prime numbers and emojis"] pub struct SignatureCompressor { # [doc = " Prime number assignments (2 = most common, higher primes = rarer)"] pub prime_assignments : HashMap < String , u64 > , # [doc = " Emoji assignments for visual representation"] pub emoji_assignments : HashMap < String , String > , # [doc = " Frequency tracking for optimal prime assignment"] pub frequency_map : HashMap < String , u64 > , # [doc = " Next available prime"] pub next_prime : u64 , }
}