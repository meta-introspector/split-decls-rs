// Generated macro for arm_comma (function)
macro_rules! Depcrate_matchesarm_comma {
() => {
// Module: crate::matches
// Provides: {"arm_comma"}
// Dependencies: {}
fn arm_comma (config : & Config , body : & ast :: Expr , is_last : bool) -> & 'static str { if is_last && config . trailing_comma () == SeparatorTactic :: Never { "" } else if config . match_block_trailing_comma () { "," } else if let ast :: ExprKind :: Block (ref block , _) = body . kind { if let ast :: BlockCheckMode :: Default = block . rules { "" } else { "," } } else { "," } }
};
}
