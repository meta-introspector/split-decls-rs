// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'de , T > De < T > where De < T > : serde :: Deserialize < 'de > { # [doc = " Consumes this wrapper, returning the deserialized value."] # [inline (always)] pub fn into_inner (self) -> T { self . 0 } }
};
}
