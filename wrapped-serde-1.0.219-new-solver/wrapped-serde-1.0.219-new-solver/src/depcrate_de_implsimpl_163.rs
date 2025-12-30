// Generated macro for impl_163 (impl)
macro_rules! Depcrate_de_implsimpl_163 {
() => {
// Module: crate::de::impls
// Provides: {"impl_163"}
// Dependencies: {}
# [cfg (feature = "unstable")] # [cfg_attr (docsrs , doc (cfg (feature = "unstable")))] impl < 'de > Deserialize < 'de > for ! { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Err (Error :: custom ("cannot deserialize `!`")) } }
};
}
