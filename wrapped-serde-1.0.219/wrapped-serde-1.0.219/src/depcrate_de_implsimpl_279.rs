// Generated macro for impl_279 (impl)
macro_rules! Depcrate_de_implsimpl_279 {
() => {
// Module: crate::de::impls
// Provides: {"impl_279"}
// Dependencies: {}
impl < 'de , Idx > Deserialize < 'de > for RangeTo < Idx > where Idx : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let end = tri ! (deserializer . deserialize_struct ("RangeTo" , range_to :: FIELDS , range_to :: RangeToVisitor { expecting : "struct RangeTo" , phantom : PhantomData , } ,)) ; Ok (.. end) } }
};
}
