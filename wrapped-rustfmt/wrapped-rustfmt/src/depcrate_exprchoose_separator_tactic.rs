// Generated macro for choose_separator_tactic (function)
macro_rules! Depcrate_exprchoose_separator_tactic {
() => {
// Module: crate::expr
// Provides: {"choose_separator_tactic"}
// Dependencies: {}
fn choose_separator_tactic (context : & RewriteContext < '_ > , span : Span) -> Option < SeparatorTactic > { if context . inside_macro () { if span_ends_with_comma (context , span) { Some (SeparatorTactic :: Always) } else { Some (SeparatorTactic :: Never) } } else { None } }
};
}
