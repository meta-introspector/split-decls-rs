// Generated macro for impl_111 (impl)
macro_rules! Depcrate_serdeimpl_111 {
() => {
// Module: crate::serde
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for VersionReq { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct VersionReqVisitor ; impl < 'de > Visitor < 'de > for VersionReqVisitor { type Value = VersionReq ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("semver version") } fn visit_str < E > (self , string : & str) -> Result < Self :: Value , E > where E : Error , { string . parse () . map_err (Error :: custom) } } deserializer . deserialize_str (VersionReqVisitor) } }
};
}
