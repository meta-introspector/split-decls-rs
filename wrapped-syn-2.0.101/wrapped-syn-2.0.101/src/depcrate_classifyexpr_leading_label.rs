// Generated macro for expr_leading_label (function)
macro_rules! Depcrate_classifyexpr_leading_label {
() => {
// Module: crate::classify
// Provides: {"expr_leading_label"}
// Dependencies: {}
# [doc = " Whether the expression's first token is the label of a loop/block."] # [cfg (all (feature = "printing" , feature = "full"))] pub (crate) fn expr_leading_label (mut expr : & Expr) -> bool { loop { match expr { Expr :: Block (e) => return e . label . is_some () , Expr :: ForLoop (e) => return e . label . is_some () , Expr :: Loop (e) => return e . label . is_some () , Expr :: While (e) => return e . label . is_some () , Expr :: Assign (e) => expr = & e . left , Expr :: Await (e) => expr = & e . base , Expr :: Binary (e) => expr = & e . left , Expr :: Call (e) => expr = & e . func , Expr :: Cast (e) => expr = & e . expr , Expr :: Field (e) => expr = & e . base , Expr :: Index (e) => expr = & e . expr , Expr :: MethodCall (e) => expr = & e . receiver , Expr :: Range (e) => match & e . start { Some (start) => expr = start , None => return false , } , Expr :: Try (e) => expr = & e . expr , Expr :: Array (_) | Expr :: Async (_) | Expr :: Break (_) | Expr :: Closure (_) | Expr :: Const (_) | Expr :: Continue (_) | Expr :: Group (_) | Expr :: If (_) | Expr :: Infer (_) | Expr :: Let (_) | Expr :: Lit (_) | Expr :: Macro (_) | Expr :: Match (_) | Expr :: Paren (_) | Expr :: Path (_) | Expr :: RawAddr (_) | Expr :: Reference (_) | Expr :: Repeat (_) | Expr :: Return (_) | Expr :: Struct (_) | Expr :: TryBlock (_) | Expr :: Tuple (_) | Expr :: Unary (_) | Expr :: Unsafe (_) | Expr :: Verbatim (_) | Expr :: Yield (_) => return false , } } }
};
}
