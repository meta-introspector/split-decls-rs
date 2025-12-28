use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Defines big-endian serialization."] # [doc = ""] # [doc = " Note that this type has no value constructor. It is used purely at the"] # [doc = " type level."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Write and read `u32` numbers in big endian order:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use byteorder::{ByteOrder, BigEndian};"] # [doc = ""] # [doc = " let mut buf = [0; 4];"] # [doc = " BigEndian::write_u32(&mut buf, 1_000_000);"] # [doc = " assert_eq!(1_000_000, BigEndian::read_u32(&buf));"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] pub enum BigEndian { }
}