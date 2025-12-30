// Generated macro for impl_63 (impl)
macro_rules! Depcrate_seqimpl_63 {
() => {
// Module: crate::seq
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'access , 'de > Seq < 'access , 'de > { pub (crate) fn new < A > (seq : A) -> Self where A : SeqAccess < 'de > + 'access , { Seq { erased : Box :: new (seq) , } } # [doc = " Shorthand for `T::deserialize(serde::de::value::SeqAccessDeserializer::new(self))`."] pub fn deserialize < T > (self) -> Result < T , Error > where T : Deserialize < 'de > , { T :: deserialize (serde :: de :: value :: SeqAccessDeserializer :: new (self)) } }
};
}
