// Generated macro for impl_193 (impl)
macro_rules! Depcrate_fieldimpl_193 {
() => {
// Module: crate::field
// Provides: {"impl_193"}
// Dependencies: {}
impl < T > Value for DebugValue < T > where T : fmt :: Debug , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_debug (key , & self . 0) } }
};
}
