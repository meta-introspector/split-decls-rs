// Generated macro for impl_183 (impl)
macro_rules! Depcrate_fieldimpl_183 {
() => {
// Module: crate::field
// Provides: {"impl_183"}
// Dependencies: {}
impl < T : ? Sized > Value for Box < T > where T : Value , { # [inline] fn record (& self , key : & Field , visitor : & mut dyn Visit) { self . as_ref () . record (key , visitor) } }
};
}
