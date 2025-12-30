// Generated macro for impl_19 (impl)
macro_rules! Depcrate_dataimpl_19 {
() => {
// Module: crate::data
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'computed > Drop for Label < 'computed > { fn drop (& mut self) { # [cfg (feature = "alloc")] { if let Some (owned) = self . backing_field_owned { drop (unsafe { Box :: from_raw (owned) }) ; } } } }
};
}
