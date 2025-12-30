// Generated macro for impl_227 (impl)
macro_rules! Depcrate_de_implsimpl_227 {
() => {
// Module: crate::de::impls
// Provides: {"impl_227"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for [T ; 0] { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_tuple (0 , ArrayVisitor :: < [T ; 0] > :: new ()) } }
};
}
