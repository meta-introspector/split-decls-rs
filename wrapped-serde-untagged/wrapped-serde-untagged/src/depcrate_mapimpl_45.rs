// Generated macro for impl_45 (impl)
macro_rules! Depcrate_mapimpl_45 {
() => {
// Module: crate::map
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'access , 'de > Map < 'access , 'de > { pub (crate) fn new < A > (map : A) -> Self where A : MapAccess < 'de > + 'access , { Map { erased : Box :: new (map) , } } # [doc = " Shorthand for `T::deserialize(serde::de::value::MapAccessDeserializer::new(self))`."] pub fn deserialize < T > (self) -> Result < T , Error > where T : Deserialize < 'de > , { T :: deserialize (serde :: de :: value :: MapAccessDeserializer :: new (self)) } }
};
}
