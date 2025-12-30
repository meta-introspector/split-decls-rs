// Generated macro for impl_274 (impl)
macro_rules! Depcrate_de_implsimpl_274 {
() => {
// Module: crate::de::impls
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'de , Idx > Deserialize < 'de > for Range < Idx > where Idx : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let (start , end) = tri ! (deserializer . deserialize_struct ("Range" , range :: FIELDS , range :: RangeVisitor { expecting : "struct Range" , phantom : PhantomData , } ,)) ; Ok (start .. end) } }
};
}
