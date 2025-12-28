use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Wait until a counter is no longer shared, then return its value."] fn wait_for_counter (mut counter : Arc < AtomicUsize >) -> usize { use std :: { thread , time } ; for _ in 0 .. 60 { counter = match Arc :: try_unwrap (counter) { Ok (counter) => return counter . into_inner () , Err (counter) => { thread :: sleep (time :: Duration :: from_secs (1)) ; counter } } ; } panic ! ("Counter is still shared!") ; }
}