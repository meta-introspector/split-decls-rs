use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: addr");
# [cfg (feature = "alloc")] # [inline (always)] fn addr < T > (x : * const T) -> usize { # [allow (clippy :: useless_transmute , clippy :: transmutes_expressible_as_ptr_casts)] unsafe { core :: mem :: transmute (x) } }
}