// Generated macro for impl_31 (impl)
macro_rules! Depcrate_field_delimitedimpl_31 {
() => {
// Module: crate::field::delimited
// Provides: {"impl_31"}
// Dependencies: {}
impl < D , V > VisitOutput < fmt :: Result > for VisitDelimited < D , V > where V : VisitFmt , D : AsRef < str > , { fn finish (self) -> fmt :: Result { self . err ? ; self . inner . finish () } }
};
}
