// Generated macro for impl_275 (impl)
macro_rules! Depcrate_map2d_serdeimpl_275 {
() => {
// Module: crate::map2d::serde
// Provides: {"impl_275"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "serde")] impl < 'a , K0 , K1 , V > Serialize for ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , K1 : ZeroMapKV < 'a > + Serialize + ? Sized + Ord , V : ZeroMapKV < 'a > + Serialize + ? Sized , K0 :: Container : Serialize , K1 :: Container : Serialize , V :: Container : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { ZeroMap2d :: < K0 , K1 , V > :: from (* self) . serialize (serializer) } }
};
}
