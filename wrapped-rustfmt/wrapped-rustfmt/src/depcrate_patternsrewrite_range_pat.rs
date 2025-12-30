// Generated macro for rewrite_range_pat (function)
macro_rules! Depcrate_patternsrewrite_range_pat {
() => {
// Module: crate::patterns
// Provides: {"rewrite_range_pat"}
// Dependencies: {}
pub (crate) fn rewrite_range_pat < T : Rewrite > (context : & RewriteContext < '_ > , shape : Shape , lhs : & Option < ptr :: P < T > > , rhs : & Option < ptr :: P < T > > , end_kind : & rustc_span :: source_map :: Spanned < RangeEnd > , span : Span ,) -> RewriteResult { let infix = match end_kind . node { RangeEnd :: Included (RangeSyntax :: DotDotDot) => "..." , RangeEnd :: Included (RangeSyntax :: DotDotEq) => "..=" , RangeEnd :: Excluded => ".." , } ; let infix = if context . config . spaces_around_ranges () { let lhs_spacing = match lhs { None => "" , Some (_) => " " , } ; let rhs_spacing = match rhs { None => "" , Some (_) => " " , } ; format ! ("{lhs_spacing}{infix}{rhs_spacing}") } else { infix . to_owned () } ; let lspan = span . with_hi (end_kind . span . lo ()) ; let rspan = span . with_lo (end_kind . span . hi ()) ; rewrite_pair (& RangeOperand { operand : lhs , span : lspan , } , & RangeOperand { operand : rhs , span : rspan , } , PairParts :: infix (& infix) , context , shape , SeparatorPlace :: Front ,) }
};
}
