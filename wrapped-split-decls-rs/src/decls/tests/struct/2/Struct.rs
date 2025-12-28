use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (PartialEq , Clone , Debug , Encodable_NoContext , Decodable_NoContext)] struct Struct { a : () , b : u8 , c : u16 , d : u32 , e : u64 , f : usize , g : i8 , h : i16 , i : i32 , j : i64 , k : isize , l : char , m : String , p : bool , q : Option < u32 > , }
}