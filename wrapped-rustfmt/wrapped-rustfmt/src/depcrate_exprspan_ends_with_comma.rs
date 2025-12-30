// Generated macro for span_ends_with_comma (function)
macro_rules! Depcrate_exprspan_ends_with_comma {
() => {
// Module: crate::expr
// Provides: {"span_ends_with_comma"}
// Dependencies: {}
# [doc = " Returns `true` if a function call or a method call represented by the given span ends with a"] # [doc = " trailing comma. This function is used when rewriting macro, as adding or removing a trailing"] # [doc = " comma from macro can potentially break the code."] pub (crate) fn span_ends_with_comma (context : & RewriteContext < '_ > , span : Span) -> bool { let mut result : bool = Default :: default () ; let mut prev_char : char = Default :: default () ; let closing_delimiters = & [')' , '}' , ']'] ; for (kind , c) in CharClasses :: new (context . snippet (span) . chars ()) { match c { _ if kind . is_comment () || c . is_whitespace () => continue , c if closing_delimiters . contains (& c) => { result &= ! closing_delimiters . contains (& prev_char) ; } ',' => result = true , _ => result = false , } prev_char = c ; } result }
};
}
