// Generated macro for requires_comma_to_be_match_arm (function)
macro_rules! Depcrate_classifyrequires_comma_to_be_match_arm {
() => {
// Module: crate::classify
// Provides: {"requires_comma_to_be_match_arm"}
// Dependencies: {}
# [cfg (feature = "full")] pub (crate) fn requires_comma_to_be_match_arm (expr : & Expr) -> bool { match expr { Expr :: If (_) | Expr :: Match (_) | Expr :: Block (_) | Expr :: Unsafe (_) | Expr :: While (_) | Expr :: Loop (_) | Expr :: ForLoop (_) | Expr :: TryBlock (_) | Expr :: Const (_) => false , Expr :: Array (_) | Expr :: Assign (_) | Expr :: Async (_) | Expr :: Await (_) | Expr :: Binary (_) | Expr :: Break (_) | Expr :: Call (_) | Expr :: Cast (_) | Expr :: Closure (_) | Expr :: Continue (_) | Expr :: Field (_) | Expr :: Group (_) | Expr :: Index (_) | Expr :: Infer (_) | Expr :: Let (_) | Expr :: Lit (_) | Expr :: Macro (_) | Expr :: MethodCall (_) | Expr :: Paren (_) | Expr :: Path (_) | Expr :: Range (_) | Expr :: RawAddr (_) | Expr :: Reference (_) | Expr :: Repeat (_) | Expr :: Return (_) | Expr :: Struct (_) | Expr :: Try (_) | Expr :: Tuple (_) | Expr :: Unary (_) | Expr :: Yield (_) | Expr :: Verbatim (_) => true , } }
};
}
