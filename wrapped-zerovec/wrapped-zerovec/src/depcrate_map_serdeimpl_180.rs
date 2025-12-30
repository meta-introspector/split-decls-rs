// Generated macro for impl_180 (impl)
macro_rules! Depcrate_map_serdeimpl_180 {
() => {
// Module: crate::map::serde
// Provides: {"impl_180"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "serde")] impl < 'a , K , V > Serialize for ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , V : ZeroMapKV < 'a > + Serialize + ? Sized , K :: Container : Serialize , V :: Container : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { ZeroMap :: < K , V > :: from (* self) . serialize (serializer) } }
};
}
