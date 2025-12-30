// Generated macro for impl_202 (impl)
macro_rules! Depcrate_fieldimpl_202 {
() => {
// Module: crate::field
// Provides: {"impl_202"}
// Dependencies: {}
impl < T : Value > Value for Option < T > { fn record (& self , key : & Field , visitor : & mut dyn Visit) { if let Some (v) = & self { v . record (key , visitor) } } }
};
}
