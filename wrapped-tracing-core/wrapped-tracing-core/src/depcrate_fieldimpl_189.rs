// Generated macro for impl_189 (impl)
macro_rules! Depcrate_fieldimpl_189 {
() => {
// Module: crate::field
// Provides: {"impl_189"}
// Dependencies: {}
impl < T > Value for DisplayValue < T > where T : fmt :: Display , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_debug (key , self) } }
};
}
