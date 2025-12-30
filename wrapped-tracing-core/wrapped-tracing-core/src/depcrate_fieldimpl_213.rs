// Generated macro for impl_213 (impl)
macro_rules! Depcrate_fieldimpl_213 {
() => {
// Module: crate::field
// Provides: {"impl_213"}
// Dependencies: {}
impl fmt :: Display for FieldSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . names . iter () . map (display)) . finish () } }
};
}
