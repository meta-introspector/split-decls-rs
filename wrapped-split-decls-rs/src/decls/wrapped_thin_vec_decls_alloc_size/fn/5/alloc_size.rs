use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: alloc_size");
# [doc = " Gets the size necessary to allocate a `ThinVec<T>` with the give capacity."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic if isize::MAX is overflowed at any point."] fn alloc_size < T > (cap : usize) -> usize { let header_size = mem :: size_of :: < Header > () as isize ; let padding = padding :: < T > () as isize ; let data_size = if mem :: size_of :: < T > () == 0 { 0 } else { let cap : isize = cap . try_into () . unwrap_cap_overflow () ; let elem_size = mem :: size_of :: < T > () as isize ; elem_size . checked_mul (cap) . unwrap_cap_overflow () } ; let final_size = data_size . checked_add (header_size + padding) . unwrap_cap_overflow () ; final_size as usize }
}