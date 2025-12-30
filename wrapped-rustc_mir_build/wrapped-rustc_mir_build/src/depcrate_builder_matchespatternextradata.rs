// Generated macro for PatternExtraData (struct)
macro_rules! Depcrate_builder_matchesPatternExtraData {
() => {
// Module: crate::builder::matches
// Provides: {"PatternExtraData"}
// Dependencies: {}
# [doc = " Data extracted from a pattern that doesn't affect which branch is taken. Collected during"] # [doc = " pattern simplification and not mutated later."] # [derive (Debug , Clone)] struct PatternExtraData < 'tcx > { # [doc = " [`Span`] of the original pattern."] span : Span , # [doc = " Bindings that must be established."] bindings : Vec < SubpatternBindings < 'tcx > > , # [doc = " Types that must be asserted."] ascriptions : Vec < Ascription < 'tcx > > , # [doc = " Whether this corresponds to a never pattern."] is_never : bool , }
};
}
