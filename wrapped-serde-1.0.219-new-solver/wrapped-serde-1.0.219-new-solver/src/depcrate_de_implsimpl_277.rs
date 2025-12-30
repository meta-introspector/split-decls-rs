// Generated macro for impl_277 (impl)
macro_rules! Depcrate_de_implsimpl_277 {
() => {
// Module: crate::de::impls
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'de , Idx > Deserialize < 'de > for RangeFrom < Idx > where Idx : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let start = tri ! (deserializer . deserialize_struct ("RangeFrom" , range_from :: FIELDS , range_from :: RangeFromVisitor { expecting : "struct RangeFrom" , phantom : PhantomData , } ,)) ; Ok (start ..) } }
};
}
