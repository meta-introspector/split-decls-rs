use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " How pointers are represented in a given address space"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct PointerSpec { # [doc = " The size of the bitwise representation of the pointer."] pointer_size : Size , # [doc = " The alignment of pointers for this address space"] pointer_align : AbiAlign , # [doc = " The size of the value a pointer can be offset by in this address space."] pointer_offset : Size , # [doc = " Pointers into this address space contain extra metadata"] # [doc = " FIXME(workingjubilee): Consider adequately reflecting this in the compiler?"] _is_fat : bool , }
}