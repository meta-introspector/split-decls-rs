// Generated macro for impl_345 (impl)
macro_rules! Depcrate_config_optionsimpl_345 {
() => {
// Module: crate::config::options
// Provides: {"impl_345"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for IgnoreList { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct HashSetVisitor ; impl < 'v > Visitor < 'v > for HashSetVisitor { type Value = HashSet < PathBuf > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence of path") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'v > , { let mut path_set = HashSet :: new () ; while let Some (elem) = seq . next_element () ? { path_set . insert (elem) ; } Ok (path_set) } } Ok (IgnoreList { path_set : deserializer . deserialize_seq (HashSetVisitor) ? , rustfmt_toml_path : PathBuf :: new () , }) } }
};
}
