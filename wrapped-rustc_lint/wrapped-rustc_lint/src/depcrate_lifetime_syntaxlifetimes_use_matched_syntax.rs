// Generated macro for lifetimes_use_matched_syntax (function)
macro_rules! Depcrate_lifetime_syntaxlifetimes_use_matched_syntax {
() => {
// Module: crate::lifetime_syntax
// Provides: {"lifetimes_use_matched_syntax"}
// Dependencies: {}
fn lifetimes_use_matched_syntax (input_info : & [Info < '_ >] , output_info : & [Info < '_ >]) -> bool { let mut syntax_counts = LifetimeSyntaxCategories :: < usize > :: default () ; for info in input_info . iter () . chain (output_info) { if let Some (category) = info . lifetime_syntax_category () { * syntax_counts . select (category) += 1 ; } } tracing :: debug ! (? syntax_counts) ; matches ! (syntax_counts , LifetimeSyntaxCategories { hidden : _ , elided : 0 , named : 0 } | LifetimeSyntaxCategories { hidden : 0 , elided : _ , named : 0 } | LifetimeSyntaxCategories { hidden : 0 , elided : 0 , named : _ }) }
};
}
