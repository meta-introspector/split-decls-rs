// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'de , T > DerefMut for Serde < T > where De < T > : Deserialize < 'de > , for < 'a > Ser < 'a , T > : Serialize { fn deref_mut (& mut self) -> & mut T { & mut self . 0 } }
};
}
