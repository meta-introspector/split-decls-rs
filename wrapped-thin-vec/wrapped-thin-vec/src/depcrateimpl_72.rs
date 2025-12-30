// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < T > From < Vec < T > > for ThinVec < T > { # [doc = " Convert a `std::Vec` into a `ThinVec`."] # [doc = ""] # [doc = " **NOTE:** this must reallocate to change the layout!"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " let b: Vec<i32> = vec![1, 2, 3];"] # [doc = " assert_eq!(ThinVec::from(b), thin_vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : Vec < T >) -> Self { s . into_iter () . collect () } }
};
}
