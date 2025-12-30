// Generated macro for should_add_parens (function)
macro_rules! Depcrate_chainsshould_add_parens {
() => {
// Module: crate::chains
// Provides: {"should_add_parens"}
// Dependencies: {}
# [doc = " Whether a method call's receiver needs parenthesis, like"] # [doc = " ```rust,ignore"] # [doc = " || .. .method();"] # [doc = " || 1.. .method();"] # [doc = " 1. .method();"] # [doc = " ```"] # [doc = " Which all need parenthesis or a space before `.method()`."] fn should_add_parens (expr : & ast :: Expr , context : & RewriteContext < '_ >) -> bool { match expr . kind { ast :: ExprKind :: Lit (ref lit) => crate :: expr :: lit_ends_in_dot (lit , context) , ast :: ExprKind :: Closure (ref cl) => match cl . body . kind { ast :: ExprKind :: Range (_ , _ , ast :: RangeLimits :: HalfOpen) => true , ast :: ExprKind :: Lit (ref lit) => crate :: expr :: lit_ends_in_dot (lit , context) , _ => false , } , _ => false , } }
};
}
