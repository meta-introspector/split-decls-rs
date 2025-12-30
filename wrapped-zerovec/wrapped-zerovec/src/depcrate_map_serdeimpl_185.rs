// Generated macro for impl_185 (impl)
macro_rules! Depcrate_map_serdeimpl_185 {
() => {
// Module: crate::map::serde
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'de , 'a , K , V > Deserialize < 'de > for ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + Ord + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: Container : Deserialize < 'de > , V :: Container : Deserialize < 'de > , K :: OwnedType : Deserialize < 'de > , V :: OwnedType : Deserialize < 'de > , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { Err (de :: Error :: custom ("ZeroMapBorrowed cannot be deserialized from human-readable formats" ,)) } else { let deserialized : ZeroMap < 'a , K , V > = ZeroMap :: deserialize (deserializer) ? ; let keys = if let Some (keys) = deserialized . keys . zvl_as_borrowed_inner () { keys } else { return Err (de :: Error :: custom ("ZeroMapBorrowed can only deserialize in zero-copy ways" ,)) ; } ; let values = if let Some (values) = deserialized . values . zvl_as_borrowed_inner () { values } else { return Err (de :: Error :: custom ("ZeroMapBorrowed can only deserialize in zero-copy ways" ,)) ; } ; Ok (Self { keys , values }) } } }
};
}
