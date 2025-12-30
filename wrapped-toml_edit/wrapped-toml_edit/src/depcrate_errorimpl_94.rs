// Generated macro for impl_94 (impl)
macro_rules! Depcrate_errorimpl_94 {
() => {
// Module: crate::error
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "parse")] impl < 'i , S : Default > TomlSink < 'i , S > { pub (crate) fn new (source : toml_parser :: Source < 'i >) -> Self { Self { source , input : None , sink : Default :: default () , } } pub (crate) fn into_inner (self) -> S { self . sink } }
};
}
