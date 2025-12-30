// Generated macro for impl_59 (impl)
macro_rules! Depcrate_fieldimpl_59 {
() => {
// Module: crate::field
// Provides: {"impl_59"}
// Dependencies: {}
impl < F > RecordFields for & F where F : RecordFields , { fn record (& self , visitor : & mut dyn Visit) { F :: record (* self , visitor) } }
};
}
