// Generated macro for impl_214 (impl)
macro_rules! Depcrate_de_implsimpl_214 {
() => {
// Module: crate::de::impls
// Provides: {"impl_214"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for PhantomData < T > where T : ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let visitor = PhantomDataVisitor { marker : PhantomData , } ; deserializer . deserialize_unit_struct ("PhantomData" , visitor) } }
};
}
