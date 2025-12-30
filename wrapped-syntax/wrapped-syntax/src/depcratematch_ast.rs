// Generated macro for match_ast (macro)
macro_rules! Depcratematch_ast {
() => {
// Module: crate
// Provides: {"match_ast"}
// Dependencies: {}
# [doc = " Matches a `SyntaxNode` against an `ast` type."] # [doc = ""] # [doc = " # Example:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " match_ast! {"] # [doc = "     match node {"] # [doc = "         ast::CallExpr(it) => { ... },"] # [doc = "         ast::MethodCallExpr(it) => { ... },"] # [doc = "         ast::MacroCall(it) => { ... },"] # [doc = "         _ => None,"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! match_ast { (match $ node : ident { $ ($ tt : tt) * }) => { $ crate :: match_ast ! (match ($ node) { $ ($ tt) * }) } ; (match ($ node : expr) { $ ($ ($ path : ident) ::+ ($ it : pat) => $ res : expr ,) * _ => $ catch_all : expr $ (,) ? }) => { { $ (if let Some ($ it) = $ ($ path ::) + cast ($ node . clone ()) { $ res } else) * { $ catch_all } } } ; }
};
}
