// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'de , T > Serde < T > where De < T > : Deserialize < 'de > , for < 'a > Ser < 'a , T > : Serialize { # [doc = " Consumes this wrapper, returning the inner value."] # [inline (always)] pub fn into_inner (self) -> T { self . 0 } }
};
}
