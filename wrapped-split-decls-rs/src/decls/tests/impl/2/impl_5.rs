use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl Tag for Tag2 { const BITS : u32 = 2 ; fn into_usize (self) -> usize { self as _ } unsafe fn from_usize (tag : usize) -> Self { match tag { 0b00 => Tag2 :: B00 , 0b01 => Tag2 :: B01 , 0b10 => Tag2 :: B10 , 0b11 => Tag2 :: B11 , _ => unreachable ! () , } } }
}