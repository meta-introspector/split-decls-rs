use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: empty");
# [doc = " Creates an iterator that yields nothing."] pub fn empty < T , E > () -> Empty < T , E > { Empty (PhantomData , PhantomData) }
}