// Generated macro for encode_none (macro)
macro_rules! Depcrate_test_simd_returnencode_none {
() => {
// Module: crate::test_simd_return
// Provides: {"encode_none"}
// Dependencies: {}
macro_rules ! encode_none { ($ ty : ty) => { unsafe impl Encode for $ ty { const ENCODING : Encoding = Encoding :: None ; } } ; }
};
}
