// Generated macro for impl_183 (impl)
macro_rules! Depcrate_map_serdeimpl_183 {
() => {
// Module: crate::map::serde
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'a , 'de , K , V > Visitor < 'de > for ZeroMapMapVisitor < 'a , K , V > where K : ZeroMapKV < 'a > + Ord + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: OwnedType : Deserialize < 'de > , V :: OwnedType : Deserialize < 'de > , { type Value = ZeroMap < 'a , K , V > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a map produced by ZeroMap") } fn visit_seq < S > (self , mut access : S) -> Result < Self :: Value , S :: Error > where S : SeqAccess < 'de > , { let mut map = ZeroMap :: with_capacity (access . size_hint () . unwrap_or (0)) ; while let Some ((key , value)) = access . next_element :: < (K :: OwnedType , V :: OwnedType) > () ? { if map . try_append (K :: Container :: owned_as_t (& key) , V :: Container :: owned_as_t (& value) ,) . is_some () { return Err (de :: Error :: custom ("ZeroMap's keys must be sorted while deserializing" ,)) ; } } Ok (map) } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut map = ZeroMap :: with_capacity (access . size_hint () . unwrap_or (0)) ; while let Some ((key , value)) = access . next_entry :: < K :: OwnedType , V :: OwnedType > () ? { if map . try_append (K :: Container :: owned_as_t (& key) , V :: Container :: owned_as_t (& value) ,) . is_some () { return Err (de :: Error :: custom ("ZeroMap's keys must be sorted while deserializing" ,)) ; } } Ok (map) } }
};
}
