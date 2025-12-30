// Generated macro for has_explicit_parent (function)
macro_rules! Depcrate_expecthas_explicit_parent {
() => {
// Module: crate::expect
// Provides: {"has_explicit_parent"}
// Dependencies: {}
# [doc = " Convenience function that returns [`ExpectedAncestry::HasExplicitParent`] with"] # [doc = " provided name."] pub fn has_explicit_parent < S : Into < ExpectedSpan > > (span : S) -> ExpectedAncestry { ExpectedAncestry :: HasExplicitParent (span . into ()) }
};
}
