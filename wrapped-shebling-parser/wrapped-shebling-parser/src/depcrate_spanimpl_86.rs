// Generated macro for impl_86 (impl)
macro_rules! Depcrate_spanimpl_86 {
() => {
// Module: crate::span
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a > ParseSpan < 'a > { pub (crate) fn new (source : & 'a str , diags : & 'a ParseDiags) -> Self { Self { fragment : source , offset : 0 , diags , } } pub (crate) fn fragment (& self) -> & str { self . fragment } pub (crate) fn offset (& self) -> usize { self . offset } pub (crate) fn diag (& self , builder : DiagnosticBuilder) { self . diags . push (builder) ; } }
};
}
