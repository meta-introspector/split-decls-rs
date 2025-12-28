use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: from_fn");
# [doc = " Creates an iterator from a fallible function generating values."] # [doc = ""] # [doc = " ```"] # [doc = " # use fallible_iterator::{from_fn, FallibleIterator};"] # [doc = " let mut count = 0;"] # [doc = " let counter = from_fn(move || {"] # [doc = "     // Increment our count. This is why we started at zero."] # [doc = "     count += 1;"] # [doc = ""] # [doc = "     // Check to see if we've finished counting or not."] # [doc = "     if count < 6 {"] # [doc = "         Ok(Some(count))"] # [doc = "     } else if count < 7 {"] # [doc = "         Ok(None)"] # [doc = "     } else {"] # [doc = "         Err(())"] # [doc = "     }"] # [doc = " });"] # [doc = " assert_eq!(&counter.collect::<Vec<_>>().unwrap(), &[1, 2, 3, 4, 5]);"] # [doc = " ```"] # [inline] pub fn from_fn < I , E , F > (fun : F) -> FromFn < F > where F : FnMut () -> Result < Option < I > , E > , { FromFn { fun } }
}