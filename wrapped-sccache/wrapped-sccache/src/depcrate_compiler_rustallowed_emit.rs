// Generated macro for ALLOWED_EMIT (static)
macro_rules! Depcrate_compiler_rustALLOWED_EMIT {
() => {
// Module: crate::compiler::rust
// Provides: {"ALLOWED_EMIT"}
// Dependencies: {}
# [doc = " Emit types that we will cache."] static ALLOWED_EMIT : LazyLock < HashSet < & 'static str > > = LazyLock :: new (| | ["link" , "metadata" , "dep-info"] . iter () . copied () . collect ()) ;
};
}
