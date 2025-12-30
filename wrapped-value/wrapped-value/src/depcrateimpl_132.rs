// Generated macro for impl_132 (impl)
macro_rules! Depcrateimpl_132 {
() => {
// Module: crate
// Provides: {"impl_132"}
// Dependencies: {}
impl From < f64 > for ConstValue { # [inline] fn from (f : f64) -> Self { Number :: from_f64 (f) . map_or (ConstValue :: Null , ConstValue :: Number) } }
};
}
