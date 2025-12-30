// Generated macro for impl_301 (impl)
macro_rules! Depcrate_serdeimpl_301 {
() => {
// Module: crate::serde
// Provides: {"impl_301"}
// Dependencies: {}
impl < 'd , K , V , H > Visitor < 'd > for HashIndexVisitor < K , V , H > where K : Deserialize < 'd > + Eq + Hash , V : Deserialize < 'd > , H : BuildHasher + Default , { type Value = HashIndex < K , V , H > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("HashIndex") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'd > , { let hashindex = HashIndex :: with_capacity_and_hasher (access . size_hint () . unwrap_or (0) . min (MAX_CAPACITY) , H :: default () ,) ; while let Some ((key , val)) = access . next_entry () ? { let _result = hashindex . insert_sync (key , val) ; } Ok (hashindex) } }
};
}
