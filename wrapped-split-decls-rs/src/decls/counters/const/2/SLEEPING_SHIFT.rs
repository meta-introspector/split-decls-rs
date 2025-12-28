use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Bits to shift to select the sleeping threads"] # [doc = " (used with `select_bits`)."] # [allow (clippy :: erasing_op)] const SLEEPING_SHIFT : usize = 0 * THREADS_BITS ;
}