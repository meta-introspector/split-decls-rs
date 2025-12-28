use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn hash < T : HashStable < () > > (t : & T) -> Hash128 { let mut h = StableHasher :: new () ; let ctx = & mut () ; t . hash_stable (ctx , & mut h) ; h . finish () }
}