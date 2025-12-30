// Generated macro for impl_59 (impl)
macro_rules! Depcrate_hashmap_serdeimpl_59 {
() => {
// Module: crate::hashmap::serde
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'de , 'a , K , V > Deserialize < 'de > for ZeroHashMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: Container : Deserialize < 'de > , V :: Container : Deserialize < 'de > , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let (displacements , keys , values) : (ZeroVec < (u32 , u32) > , K :: Container , V :: Container) = Deserialize :: deserialize (deserializer) ? ; if keys . zvl_len () != values . zvl_len () { return Err (de :: Error :: custom ("Mismatched key and value sizes in ZeroHashMap" ,)) ; } if displacements . zvl_len () != keys . zvl_len () { return Err (de :: Error :: custom ("Mismatched displacements and key, value sizes in ZeroHashMap" ,)) ; } Ok (Self { displacements , keys , values , }) } }
};
}
