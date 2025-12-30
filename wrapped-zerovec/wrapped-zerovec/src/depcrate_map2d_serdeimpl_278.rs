// Generated macro for impl_278 (impl)
macro_rules! Depcrate_map2d_serdeimpl_278 {
() => {
// Module: crate::map2d::serde
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a , 'de , K0 , K1 , V > Visitor < 'de > for ZeroMap2dMapVisitor < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Ord + ? Sized + Ord , K1 : ZeroMapKV < 'a > + Ord + ? Sized + Ord , V : ZeroMapKV < 'a > + ? Sized , K1 :: Container : Deserialize < 'de > , V :: Container : Deserialize < 'de > , K0 :: OwnedType : Deserialize < 'de > , K1 :: OwnedType : Deserialize < 'de > , V :: OwnedType : Deserialize < 'de > , { type Value = ZeroMap2d < 'a , K0 , K1 , V > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a map produced by ZeroMap2d") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut map = ZeroMap2d :: with_capacity (access . size_hint () . unwrap_or (0)) ; while let Some ((key0 , inner_map)) = access . next_entry :: < K0 :: OwnedType , TupleVecMap < K1 :: OwnedType , V :: OwnedType > > () ? { for (key1 , value) in inner_map . entries . iter () { if map . try_append (K0 :: Container :: owned_as_t (& key0) , K1 :: Container :: owned_as_t (key1) , V :: Container :: owned_as_t (value) ,) . is_some () { return Err (de :: Error :: custom ("ZeroMap2d's keys must be sorted while deserializing" ,)) ; } } } Ok (map) } }
};
}
