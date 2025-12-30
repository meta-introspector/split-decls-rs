// Generated macro for impl_32 (impl)
macro_rules! Depcrate_field_delimitedimpl_32 {
() => {
// Module: crate::field::delimited
// Provides: {"impl_32"}
// Dependencies: {}
impl < D , V > VisitFmt for VisitDelimited < D , V > where V : VisitFmt , D : AsRef < str > , { fn writer (& mut self) -> & mut dyn fmt :: Write { self . inner . writer () } }
};
}
