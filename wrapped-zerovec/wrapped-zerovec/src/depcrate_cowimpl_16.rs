// Generated macro for impl_16 (impl)
macro_rules! Depcrate_cowimpl_16 {
() => {
// Module: crate::cow
// Provides: {"impl_16"}
// Dependencies: {}
impl Clone for RawVarZeroCow { fn clone (& self) -> Self { # [cfg (feature = "alloc")] if self . is_owned () { let b : Box < [u8] > = self . as_bytes () . into () ; let b = ManuallyDrop :: new (b) ; let buf : NonNull < [u8] > = (& * * b) . into () ; return Self { buf , owned : true , } ; } Self { buf : self . buf , # [cfg (feature = "alloc")] owned : false , } } }
};
}
