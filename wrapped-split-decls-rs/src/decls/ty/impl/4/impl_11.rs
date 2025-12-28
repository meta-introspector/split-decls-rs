use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Layout < 'a > { pub fn fields (self) -> & 'a FieldsShape < FieldIdx > { & self . 0 . 0 . fields } pub fn variants (self) -> & 'a Variants < FieldIdx , VariantIdx > { & self . 0 . 0 . variants } pub fn backend_repr (self) -> BackendRepr { self . 0 . 0 . backend_repr } pub fn largest_niche (self) -> Option < Niche > { self . 0 . 0 . largest_niche } pub fn align (self) -> AbiAlign { self . 0 . 0 . align } pub fn size (self) -> Size { self . 0 . 0 . size } pub fn max_repr_align (self) -> Option < Align > { self . 0 . 0 . max_repr_align } pub fn unadjusted_abi_align (self) -> Align { self . 0 . 0 . unadjusted_abi_align } }
}