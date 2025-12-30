// Generated macro for impl_662 (impl)
macro_rules! Depcrate_ule_optionimpl_662 {
() => {
// Module: crate::ule::option
// Provides: {"impl_662"}
// Dependencies: {}
impl < U : VarULE + ? Sized + Ord > Ord for OptionVarULE < U > { fn cmp (& self , other : & Self) -> Ordering { self . as_ref () . cmp (& other . as_ref ()) } }
};
}
