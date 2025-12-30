// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < T : PartialEq > ThinVec < T > { # [doc = " Removes consecutive repeated elements in the vector."] # [doc = ""] # [doc = " If the vector is sorted, this removes all duplicates."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [cfg_attr (not (feature = "gecko-ffi") , doc = "```")] # [cfg_attr (feature = "gecko-ffi" , doc = "```ignore")] # [doc = " # #[macro_use] extern crate thin_vec;"] # [doc = " # fn main() {"] # [doc = " let mut vec = thin_vec![1, 2, 2, 3, 2];"] # [doc = ""] # [doc = " vec.dedup();"] # [doc = ""] # [doc = " assert_eq!(vec, [1, 2, 3, 2]);"] # [doc = " # }"] # [doc = " ```"] pub fn dedup (& mut self) { self . dedup_by (| a , b | a == b) } }
};
}
