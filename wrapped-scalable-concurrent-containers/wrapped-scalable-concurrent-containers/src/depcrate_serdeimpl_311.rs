// Generated macro for impl_311 (impl)
macro_rules! Depcrate_serdeimpl_311 {
() => {
// Module: crate::serde
// Provides: {"impl_311"}
// Dependencies: {}
impl < 'd , K , V > Visitor < 'd > for TreeIndexVisitor < K , V > where K : 'static + Clone + Deserialize < 'd > + Ord , V : 'static + Clone + Deserialize < 'd > , { type Value = TreeIndex < K , V > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("TreeIndex") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'd > , { let treeindex = TreeIndex :: default () ; while let Some ((key , val)) = access . next_entry () ? { let _result = treeindex . insert_sync (key , val) ; } Ok (treeindex) } }
};
}
