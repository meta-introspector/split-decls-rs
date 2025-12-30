// Generated macro for vectors (macro)
macro_rules! Depcrate_convert_slicesvectors {
() => {
// Module: crate::convert::slices
// Provides: {"vectors"}
// Dependencies: {}
macro_rules ! vectors { ($ ($ t : ty) *) => ($ (vectors_internal ! ($ t) ; vectors_internal ! (MaybeUninit <$ t >) ;) *) }
};
}
