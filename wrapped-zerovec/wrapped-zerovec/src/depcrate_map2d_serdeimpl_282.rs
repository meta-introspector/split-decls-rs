// Generated macro for impl_282 (impl)
macro_rules! Depcrate_map2d_serdeimpl_282 {
() => {
// Module: crate::map2d::serde
// Provides: {"impl_282"}
// Dependencies: {}
impl < 'de , K1 , V > Visitor < 'de > for TupleVecMapVisitor < K1 , V > where K1 : Deserialize < 'de > , V : Deserialize < 'de > , { type Value = TupleVecMap < K1 , V > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("an inner map produced by ZeroMap2d") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut result = Vec :: with_capacity (access . size_hint () . unwrap_or (0)) ; while let Some ((key1 , value)) = access . next_entry :: < K1 , V > () ? { result . push ((key1 , value)) ; } Ok (TupleVecMap { entries : result }) } }
};
}
