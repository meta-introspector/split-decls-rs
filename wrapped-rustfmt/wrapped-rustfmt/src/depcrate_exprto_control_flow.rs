// Generated macro for to_control_flow (function)
macro_rules! Depcrate_exprto_control_flow {
() => {
// Module: crate::expr
// Provides: {"to_control_flow"}
// Dependencies: {}
fn to_control_flow (expr : & ast :: Expr , expr_type : ExprType) -> Option < ControlFlow < '_ > > { match expr . kind { ast :: ExprKind :: If (ref cond , ref if_block , ref else_block) => { let (pat , cond) = extract_pats_and_cond (cond) ; Some (ControlFlow :: new_if (cond , pat , if_block , else_block . as_ref () . map (| e | & * * e) , expr_type == ExprType :: SubExpression , false , expr . span ,)) } ast :: ExprKind :: ForLoop { ref pat , ref iter , ref body , label , kind , } => Some (ControlFlow :: new_for (pat , iter , body , label , expr . span , kind ,)) , ast :: ExprKind :: Loop (ref block , label , _) => { Some (ControlFlow :: new_loop (block , label , expr . span)) } ast :: ExprKind :: While (ref cond , ref block , label) => { let (pat , cond) = extract_pats_and_cond (cond) ; Some (ControlFlow :: new_while (pat , cond , block , label , expr . span)) } _ => None , } }
};
}
