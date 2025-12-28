use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Compares two 512-bit byte strings in constant time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use constant_time_eq::constant_time_eq_64;"] # [doc = ""] # [doc = " assert!(constant_time_eq_64(&[3; 64], &[3; 64]));"] # [doc = " assert!(!constant_time_eq_64(&[3; 64], &[7; 64]));"] # [doc = " ```"] # [inline] # [must_use] pub fn constant_time_eq_64 (a : & [u8 ; 64] , b : & [u8 ; 64]) -> bool { constant_time_eq_n (a , b) }