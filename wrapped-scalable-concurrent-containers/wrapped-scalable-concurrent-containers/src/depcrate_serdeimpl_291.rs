// Generated macro for impl_291 (impl)
macro_rules! Depcrate_serdeimpl_291 {
() => {
// Module: crate::serde
// Provides: {"impl_291"}
// Dependencies: {}
impl < 'd , K , V , H > Visitor < 'd > for HashMapVisitor < K , V , H > where K : Deserialize < 'd > + Eq + Hash , V : Deserialize < 'd > , H : BuildHasher + Default , { type Value = HashMap < K , V , H > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("HashMap") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'd > , { let hashmap = HashMap :: with_capacity_and_hasher (access . size_hint () . unwrap_or (0) . min (MAX_CAPACITY) , H :: default () ,) ; while let Some ((key , val)) = access . next_entry () ? { hashmap . upsert_sync (key , val) ; } Ok (hashmap) } }
};
}
