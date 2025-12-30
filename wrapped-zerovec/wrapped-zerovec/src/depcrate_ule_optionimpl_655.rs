// Generated macro for impl_655 (impl)
macro_rules! Depcrate_ule_optionimpl_655 {
() => {
// Module: crate::ule::option
// Provides: {"impl_655"}
// Dependencies: {}
impl < U : VarULE + ? Sized > OptionVarULE < U > { # [doc = " Obtain this as an `Option<&U>`"] pub fn as_ref (& self) -> Option < & U > { if self . 1 { unsafe { Some (U :: from_bytes_unchecked (& self . 2)) } } else { None } } }
};
}
