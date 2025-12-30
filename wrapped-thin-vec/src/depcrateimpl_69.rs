// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : Clone > From < & mut [T] > for ThinVec < T > { # [doc = " Allocate a `ThinVec<T>` and fill it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " assert_eq!(ThinVec::from(&mut [1, 2, 3][..]), thin_vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : & mut [T]) -> ThinVec < T > { s . iter () . cloned () . collect () } }
};
}
