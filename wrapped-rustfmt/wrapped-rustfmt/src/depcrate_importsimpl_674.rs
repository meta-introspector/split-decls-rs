// Generated macro for impl_674 (impl)
macro_rules! Depcrate_importsimpl_674 {
() => {
// Module: crate::imports
// Provides: {"impl_674"}
// Dependencies: {}
impl Rewrite for UseSegment { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { Ok (match self . kind { UseSegmentKind :: Ident (ref ident , Some (ref rename)) => { format ! ("{ident} as {rename}") } UseSegmentKind :: Ident (ref ident , None) => ident . clone () , UseSegmentKind :: Slf (Some (ref rename)) => format ! ("self as {rename}") , UseSegmentKind :: Slf (None) => "self" . to_owned () , UseSegmentKind :: Super (Some (ref rename)) => format ! ("super as {rename}") , UseSegmentKind :: Super (None) => "super" . to_owned () , UseSegmentKind :: Crate (Some (ref rename)) => format ! ("crate as {rename}") , UseSegmentKind :: Crate (None) => "crate" . to_owned () , UseSegmentKind :: Glob => "*" . to_owned () , UseSegmentKind :: List (ref use_tree_list) => { rewrite_nested_use_tree (context , use_tree_list , shape . offset_left_opt (1) . and_then (| s | s . sub_width_opt (1)) . unknown_error () ? ,) ? } }) } }
};
}
