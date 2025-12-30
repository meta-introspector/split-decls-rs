// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for ParseError { fn description (& self) -> & str { match self { ParseError :: VariantNotFound => { "Unable to find a variant of the given enum matching the string given. Matching \
                 can be extended with the Serialize attribute and is case sensitive." } } } }
};
}
