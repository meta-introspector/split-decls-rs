// Generated macro for impl_13 (impl)
macro_rules! Depcrate_serdeimpl_13 {
() => {
// Module: crate::serde
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Slab < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (SlabVisitor (PhantomData)) } }
};
}
