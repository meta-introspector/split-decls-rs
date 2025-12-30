// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > From < Box < [T] > > for ThinVec < T > { # [doc = " Convert a boxed slice into a vector by transferring ownership of"] # [doc = " the existing heap allocation."] # [doc = ""] # [doc = " **NOTE:** unlike `std`, this must reallocate to change the layout!"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " let b: Box<[i32]> = thin_vec![1, 2, 3].into_iter().collect();"] # [doc = " assert_eq!(ThinVec::from(b), thin_vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : Box < [T] >) -> Self { Vec :: from (s) . into_iter () . collect () } }
};
}
