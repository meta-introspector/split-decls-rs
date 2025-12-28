use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: layout");
# [doc = " Gets the layout necessary to allocate a `ThinVec<T>`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the required size overflows `isize::MAX`."] fn layout < T > (cap : usize) -> Layout { unsafe { Layout :: from_size_align_unchecked (alloc_size :: < T > (cap) , alloc_align :: < T > ()) } }
}