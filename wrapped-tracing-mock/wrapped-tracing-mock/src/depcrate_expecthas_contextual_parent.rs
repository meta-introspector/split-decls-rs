// Generated macro for has_contextual_parent (function)
macro_rules! Depcrate_expecthas_contextual_parent {
() => {
// Module: crate::expect
// Provides: {"has_contextual_parent"}
// Dependencies: {}
# [doc = " Convenience function that returns [`ExpectedAncestry::HasContextualParent`] with"] # [doc = " provided name."] pub fn has_contextual_parent < S : Into < ExpectedSpan > > (span : S) -> ExpectedAncestry { ExpectedAncestry :: HasContextualParent (span . into ()) }
};
}
