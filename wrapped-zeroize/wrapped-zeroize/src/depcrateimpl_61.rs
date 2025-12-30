// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < Z > Zeroizing < Z > where Z : Zeroize , { # [doc = " Move value inside a `Zeroizing` wrapper which ensures it will be"] # [doc = " zeroized when it's dropped."] # [inline (always)] pub fn new (value : Z) -> Self { Self (value) } }
};
}
