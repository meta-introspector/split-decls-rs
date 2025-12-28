use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Test that `new` does not compile if there is not enough alignment for the"] # [doc = " tag in the pointer."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use rustc_data_structures::tagged_ptr::{TaggedRef, Tag};"] # [doc = ""] # [doc = " #[derive(Copy, Clone, Debug, PartialEq, Eq)]"] # [doc = " enum Tag2 { B00 = 0b00, B01 = 0b01, B10 = 0b10, B11 = 0b11 };"] # [doc = ""] # [doc = " unsafe impl Tag for Tag2 {"] # [doc = "     const BITS: u32 = 2;"] # [doc = ""] # [doc = "     fn into_usize(self) -> usize { todo!() }"] # [doc = "     unsafe fn from_usize(tag: usize) -> Self { todo!() }"] # [doc = " }"] # [doc = ""] # [doc = " let value = 12u16;"] # [doc = " let reference = &value;"] # [doc = " let tag = Tag2::B01;"] # [doc = ""] # [doc = " let _ptr = TaggedRef::<_, _, true>::new(reference, tag);"] # [doc = " ```"] # [cfg (not (miri))] const _ : () = () ;
}