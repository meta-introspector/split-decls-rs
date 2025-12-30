// Generated macro for can_be_overflowed_pat (function)
macro_rules! Depcrate_patternscan_be_overflowed_pat {
() => {
// Module: crate::patterns
// Provides: {"can_be_overflowed_pat"}
// Dependencies: {}
pub (crate) fn can_be_overflowed_pat (context : & RewriteContext < '_ > , pat : & TuplePatField < '_ > , len : usize ,) -> bool { match * pat { TuplePatField :: Pat (pat) => match pat . kind { ast :: PatKind :: Path (..) | ast :: PatKind :: Tuple (..) | ast :: PatKind :: Struct (..) | ast :: PatKind :: TupleStruct (..) => context . use_block_indent () && len == 1 , ast :: PatKind :: Ref (ref p , _) | ast :: PatKind :: Box (ref p) => { can_be_overflowed_pat (context , & TuplePatField :: Pat (p) , len) } ast :: PatKind :: Expr (ref expr) => can_be_overflowed_expr (context , expr , len) , _ => false , } , TuplePatField :: Dotdot (..) => false , } }
};
}
