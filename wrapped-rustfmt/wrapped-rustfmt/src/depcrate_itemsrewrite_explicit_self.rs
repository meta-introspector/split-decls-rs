// Generated macro for rewrite_explicit_self (function)
macro_rules! Depcrate_itemsrewrite_explicit_self {
() => {
// Module: crate::items
// Provides: {"rewrite_explicit_self"}
// Dependencies: {}
fn rewrite_explicit_self (context : & RewriteContext < '_ > , explicit_self : & ast :: ExplicitSelf , param_attrs : & str , span : Span , shape : Shape , has_multiple_attr_lines : bool ,) -> RewriteResult { let self_str = match explicit_self . node { ast :: SelfKind :: Region (lt , m) => { let mut_str = format_mutability (m) ; let lifetime_str = rewrite_opt_lifetime (context , lt) ? ; format ! ("&{lifetime_str}{mut_str}self") } ast :: SelfKind :: Pinned (lt , m) => { let mut_str = m . ptr_str () ; let lifetime_str = rewrite_opt_lifetime (context , lt) ? ; format ! ("&{lifetime_str}pin {mut_str} self") } ast :: SelfKind :: Explicit (ref ty , mutability) => { let type_str = ty . rewrite_result (context , Shape :: legacy (context . config . max_width () , Indent :: empty ()) ,) ? ; format ! ("{}self: {}" , format_mutability (mutability) , type_str) } ast :: SelfKind :: Value (mutability) => format ! ("{}self" , format_mutability (mutability)) , } ; Ok (combine_strs_with_missing_comments (context , param_attrs , & self_str , span , shape , ! has_multiple_attr_lines ,) ?) }
};
}
