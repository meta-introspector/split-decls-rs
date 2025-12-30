// Generated macro for impl_40 (impl)
macro_rules! Depcrate_approxeqimpl_40 {
() => {
// Module: crate::approxeq
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : ApproxEq > PartialEq < T > for ApproxEqWrapper < '_ , T > { fn eq (& self , other : & T) -> bool { self . 0 . approxeq (other , self . 1) } }
};
}
