// Generated macro for impl_234 (impl)
macro_rules! Depcrate_de_errorimpl_234 {
() => {
// Module: crate::de::error
// Provides: {"impl_234"}
// Dependencies: {}
# [cfg (feature = "parse")] impl < 'i , S : Default > TomlSink < 'i , S > { pub (crate) fn new (source : toml_parser :: Source < 'i >) -> Self { Self { source , input : None , sink : Default :: default () , } } pub (crate) fn into_inner (self) -> S { self . sink } }
};
}
