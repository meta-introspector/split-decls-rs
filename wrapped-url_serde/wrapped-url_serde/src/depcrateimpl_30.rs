// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'de , T > Deref for Serde < T > where De < T > : Deserialize < 'de > , for < 'a > Ser < 'a , T > : Serialize { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
};
}
