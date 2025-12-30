// Generated macro for impl_63 (impl)
macro_rules! Depcrate_abiimpl_63 {
() => {
// Module: crate::abi
// Provides: {"impl_63"}
// Dependencies: {}
impl ValueAbi { # [doc = " Returns `true` if the layout corresponds to an unsized type."] pub fn is_unsized (& self) -> bool { match * self { ValueAbi :: Scalar (_) | ValueAbi :: ScalarPair (..) | ValueAbi :: Vector { .. } => false , ValueAbi :: Aggregate { sized } => ! sized , } } }
};
}
