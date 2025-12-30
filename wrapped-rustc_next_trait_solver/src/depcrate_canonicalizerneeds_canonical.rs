// Generated macro for NEEDS_CANONICAL (const)
macro_rules! Depcrate_canonicalizerNEEDS_CANONICAL {
() => {
// Module: crate::canonicalizer
// Provides: {"NEEDS_CANONICAL"}
// Dependencies: {}
# [doc = " Does this have infer/placeholder/param, free regions or ReErased?"] const NEEDS_CANONICAL : TypeFlags = TypeFlags :: from_bits (TypeFlags :: HAS_INFER . bits () | TypeFlags :: HAS_PLACEHOLDER . bits () | TypeFlags :: HAS_PARAM . bits () | TypeFlags :: HAS_FREE_REGIONS . bits () | TypeFlags :: HAS_RE_ERASED . bits () ,) . unwrap () ;
};
}
