// Generated macro for impl_275 (impl)
macro_rules! Depcrate_de_implsimpl_275 {
() => {
// Module: crate::de::impls
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'de , Idx > Deserialize < 'de > for RangeInclusive < Idx > where Idx : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let (start , end) = tri ! (deserializer . deserialize_struct ("RangeInclusive" , range :: FIELDS , range :: RangeVisitor { expecting : "struct RangeInclusive" , phantom : PhantomData , } ,)) ; Ok (RangeInclusive :: new (start , end)) } }
};
}
