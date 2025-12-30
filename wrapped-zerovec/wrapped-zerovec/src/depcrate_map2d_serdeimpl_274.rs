// Generated macro for impl_274 (impl)
macro_rules! Depcrate_map2d_serdeimpl_274 {
() => {
// Module: crate::map2d::serde
// Provides: {"impl_274"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'a , 'l , K0 , K1 , V > Serialize for ZeroMap2dInnerMapSerialize < 'a , 'l , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , K1 : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , V : ZeroMapKV < 'a > + Serialize + ? Sized , K0 :: Container : Serialize , K1 :: Container : Serialize , V :: Container : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut serde_map = serializer . serialize_map (None) ? ; for (key1 , v) in self . cursor . iter1 () { K1 :: Container :: zvl_get_as_t (key1 , | k | serde_map . serialize_key (k)) ? ; V :: Container :: zvl_get_as_t (v , | v | serde_map . serialize_value (v)) ? ; } serde_map . end () } }
};
}
