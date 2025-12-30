// Generated macro for impl_199 (impl)
macro_rules! Depcrate_de_implsimpl_199 {
() => {
// Module: crate::de::impls
// Provides: {"impl_199"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a str { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (StrVisitor) } }
};
}
