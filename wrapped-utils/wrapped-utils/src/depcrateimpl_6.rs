// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < W > Ignore < W > where W : uWrite , { # [doc = " Creates a new `Ignore` adapter"] pub fn new (writer : W) -> Self { Self { writer } } # [doc = " Destroys the adapter and returns the underlying writer"] pub fn free (self) -> W { self . writer } }
};
}
