// Generated macro for impl_25 (impl)
macro_rules! Depcrate_cowimpl_25 {
() => {
// Module: crate::cow
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , V : VarULE + ? Sized + PartialEq > PartialEq for VarZeroCow < 'a , V > { fn eq (& self , other : & Self) -> bool { self . deref () . eq (other . deref ()) } }
};
}
