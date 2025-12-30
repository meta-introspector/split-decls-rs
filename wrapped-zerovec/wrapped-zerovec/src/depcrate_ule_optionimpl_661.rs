// Generated macro for impl_661 (impl)
macro_rules! Depcrate_ule_optionimpl_661 {
() => {
// Module: crate::ule::option
// Provides: {"impl_661"}
// Dependencies: {}
impl < U : VarULE + ? Sized + PartialOrd > PartialOrd for OptionVarULE < U > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . as_ref () . partial_cmp (& other . as_ref ()) } }
};
}
