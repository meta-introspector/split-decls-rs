// Generated macro for is_short_pattern_inner (function)
macro_rules! Depcrate_patternsis_short_pattern_inner {
() => {
// Module: crate::patterns
// Provides: {"is_short_pattern_inner"}
// Dependencies: {}
fn is_short_pattern_inner (context : & RewriteContext < '_ > , pat : & ast :: Pat) -> bool { match & pat . kind { ast :: PatKind :: Rest | ast :: PatKind :: Never | ast :: PatKind :: Wild | ast :: PatKind :: Err (_) => { true } ast :: PatKind :: Expr (expr) => match & expr . kind { ast :: ExprKind :: Lit (_) => true , ast :: ExprKind :: Unary (ast :: UnOp :: Neg , expr) => match & expr . kind { ast :: ExprKind :: Lit (_) => true , _ => unreachable ! () , } , ast :: ExprKind :: ConstBlock (_) | ast :: ExprKind :: Path (..) => { context . config . style_edition () <= StyleEdition :: Edition2024 } _ => unreachable ! () , } , ast :: PatKind :: Ident (_ , _ , ref pat) => pat . is_none () , ast :: PatKind :: Struct (..) | ast :: PatKind :: MacCall (..) | ast :: PatKind :: Slice (..) | ast :: PatKind :: Path (..) | ast :: PatKind :: Range (..) | ast :: PatKind :: Guard (..) => false , ast :: PatKind :: Tuple (ref subpats) => subpats . len () <= 1 , ast :: PatKind :: TupleStruct (_ , ref path , ref subpats) => { path . segments . len () <= 1 && subpats . len () <= 1 } ast :: PatKind :: Box (ref p) | PatKind :: Deref (ref p) | ast :: PatKind :: Ref (ref p , _) | ast :: PatKind :: Paren (ref p) => is_short_pattern_inner (context , & * p) , PatKind :: Or (ref pats) => pats . iter () . all (| p | is_short_pattern_inner (context , p)) , } }
};
}
