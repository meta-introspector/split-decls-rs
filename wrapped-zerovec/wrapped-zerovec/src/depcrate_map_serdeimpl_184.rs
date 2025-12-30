// Generated macro for impl_184 (impl)
macro_rules! Depcrate_map_serdeimpl_184 {
() => {
// Module: crate::map::serde
// Provides: {"impl_184"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] impl < 'de , 'a , K , V > Deserialize < 'de > for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + Ord + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: Container : Deserialize < 'de > , V :: Container : Deserialize < 'de > , K :: OwnedType : Deserialize < 'de > , V :: OwnedType : Deserialize < 'de > , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_any (ZeroMapMapVisitor :: < 'a , K , V > :: new ()) } else { let (keys , values) : (K :: Container , V :: Container) = Deserialize :: deserialize (deserializer) ? ; if keys . zvl_len () != values . zvl_len () { return Err (de :: Error :: custom ("Mismatched key and value sizes in ZeroMap" ,)) ; } debug_assert ! (keys . zvl_is_ascending ()) ; Ok (Self { keys , values }) } } }
};
}
