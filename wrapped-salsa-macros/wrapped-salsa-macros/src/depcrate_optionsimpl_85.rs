// Generated macro for impl_85 (impl)
macro_rules! Depcrate_optionsimpl_85 {
() => {
// Module: crate::options
// Provides: {"impl_85"}
// Dependencies: {}
impl < A : AllowedOptions > Options < A > { pub fn persist (& self) -> bool { cfg ! (feature = "persistence") && self . persist . is_some () } }
};
}
