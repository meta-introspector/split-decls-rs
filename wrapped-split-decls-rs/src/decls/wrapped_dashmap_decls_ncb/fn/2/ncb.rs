use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: ncb");
fn ncb (shard_amount : usize) -> usize { shard_amount . trailing_zeros () as usize }
}