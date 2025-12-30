// Generated macro for impl_272 (impl)
macro_rules! Depcrate_map2d_serdeimpl_272 {
() => {
// Module: crate::map2d::serde
// Provides: {"impl_272"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "serde")] impl < 'a , K0 , K1 , V > Serialize for ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , K1 : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , V : ZeroMapKV < 'a > + Serialize + ? Sized , K0 :: Container : Serialize , K1 :: Container : Serialize , V :: Container : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let mut serde_map = serializer . serialize_map (None) ? ; for cursor in self . iter0 () { K0 :: Container :: zvl_get_as_t (cursor . key0 () , | k | serde_map . serialize_key (k)) ? ; let inner_map = ZeroMap2dInnerMapSerialize { cursor } ; serde_map . serialize_value (& inner_map) ? ; } serde_map . end () } else { (& self . keys0 , & self . joiner , & self . keys1 , & self . values) . serialize (serializer) } } }
};
}
