// Generated macro for impl_306 (impl)
macro_rules! Depcrate_serdeimpl_306 {
() => {
// Module: crate::serde
// Provides: {"impl_306"}
// Dependencies: {}
impl < 'd , K , V , H > Visitor < 'd > for HashCacheVisitor < K , V , H > where K : Deserialize < 'd > + Eq + Hash , V : Deserialize < 'd > , H : BuildHasher + Default , { type Value = HashCache < K , V , H > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("HashCache") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'd > , { let capacity = access . size_hint () . unwrap_or (0) . min (MAX_CAPACITY) ; let hashcache = HashCache :: with_capacity_and_hasher (0 , capacity , H :: default ()) ; while let Some ((key , val)) = access . next_entry () ? { let _result = hashcache . put_sync (key , val) ; } Ok (hashcache) } }
};
}
