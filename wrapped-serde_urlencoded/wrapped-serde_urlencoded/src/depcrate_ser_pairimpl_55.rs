// Generated macro for impl_55 (impl)
macro_rules! Depcrate_ser_pairimpl_55 {
() => {
// Module: crate::ser::pair
// Provides: {"impl_55"}
// Dependencies: {}
impl Error { fn done () -> Self { Error :: Custom ("this pair has already been serialized" . into ()) } fn not_done () -> Self { Error :: Custom ("this pair has not yet been serialized" . into ()) } fn unsupported_pair () -> Self { Error :: Custom ("unsupported pair" . into ()) } }
};
}
