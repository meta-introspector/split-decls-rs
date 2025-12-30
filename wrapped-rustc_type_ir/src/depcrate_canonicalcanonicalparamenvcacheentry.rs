// Generated macro for CanonicalParamEnvCacheEntry (struct)
macro_rules! Depcrate_canonicalCanonicalParamEnvCacheEntry {
() => {
// Module: crate::canonical
// Provides: {"CanonicalParamEnvCacheEntry"}
// Dependencies: {}
# [derive_where (Clone , Debug ; I : Interner)] pub struct CanonicalParamEnvCacheEntry < I : Interner > { pub param_env : I :: ParamEnv , pub variables : Vec < I :: GenericArg > , pub variable_lookup_table : HashMap < I :: GenericArg , usize > , pub var_kinds : Vec < CanonicalVarKind < I > > , }
};
}
