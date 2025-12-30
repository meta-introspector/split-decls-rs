// Generated macro for impl_296 (impl)
macro_rules! Depcrate_serdeimpl_296 {
() => {
// Module: crate::serde
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'd , K , H > Visitor < 'd > for HashSetVisitor < K , H > where K : Deserialize < 'd > + Eq + Hash , H : BuildHasher + Default , { type Value = HashSet < K , H > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("HashSet") } fn visit_seq < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : SeqAccess < 'd > , { let hashset = HashSet :: with_capacity_and_hasher (access . size_hint () . unwrap_or (0) . min (MAX_CAPACITY) , H :: default () ,) ; while let Some (key) = access . next_element () ? { let _result = hashset . insert_sync (key) ; } Ok (hashset) } }
};
}
