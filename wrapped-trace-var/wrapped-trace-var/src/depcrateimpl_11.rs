// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " The `Fold` trait is a way to traverse an owned syntax tree and replace some"] # [doc = " of its nodes."] # [doc = ""] # [doc = " Syn provides two other syntax tree traversal traits: `Visit` which walks a"] # [doc = " shared borrow of a syntax tree, and `VisitMut` which walks an exclusive"] # [doc = " borrow of a syntax tree and can mutate it in place."] # [doc = ""] # [doc = " All three traits have a method corresponding to each type of node in Syn's"] # [doc = " syntax tree. All of these methods have default no-op implementations that"] # [doc = " simply recurse on any child nodes. We can override only those methods for"] # [doc = " which we want non-default behavior. In this case the traversal needs to"] # [doc = " transform `Expr` and `Stmt` nodes."] impl Fold for Args { fn fold_expr (& mut self , e : Expr) -> Expr { match e { Expr :: Assign (e) => { if self . should_print_expr (& e . left) { self . assign_and_print (* e . left , & e . eq_token , * e . right) } else { Expr :: Assign (fold :: fold_expr_assign (self , e)) } } Expr :: Binary (e) if is_assign_op (e . op) => { if self . should_print_expr (& e . left) { self . assign_and_print (* e . left , & e . op , * e . right) } else { Expr :: Binary (fold :: fold_expr_binary (self , e)) } } _ => fold :: fold_expr (self , e) , } } fn fold_stmt (& mut self , s : Stmt) -> Stmt { match s { Stmt :: Local (s) => { if s . init . is_some () && self . should_print_pat (& s . pat) { self . let_and_print (s) } else { Stmt :: Local (fold :: fold_local (self , s)) } } _ => fold :: fold_stmt (self , s) , } } }
};
}
