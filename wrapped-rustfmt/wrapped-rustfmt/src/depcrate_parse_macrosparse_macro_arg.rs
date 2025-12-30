// Generated macro for parse_macro_arg (function)
macro_rules! Depcrate_parse_macrosparse_macro_arg {
() => {
// Module: crate::parse::macros
// Provides: {"parse_macro_arg"}
// Dependencies: {}
fn parse_macro_arg < 'a , 'b : 'a > (parser : & 'a mut Parser < 'b >) -> Option < MacroArg > { macro_rules ! parse_macro_arg { ($ macro_arg : ident , $ nt_kind : expr , $ try_parse : expr , $ then : expr) => { let mut cloned_parser = (* parser) . clone () ; if Parser :: nonterminal_may_begin_with ($ nt_kind , & cloned_parser . token) { match $ try_parse (& mut cloned_parser) { Ok (x) => { if parser . psess . dcx () . has_errors () . is_some () { parser . psess . dcx () . reset_err_count () ; } else { * parser = cloned_parser ; return Some (MacroArg ::$ macro_arg ($ then (x) ?)) ; } } Err (e) => { e . cancel () ; parser . psess . dcx () . reset_err_count () ; } } } } ; } parse_macro_arg ! (Expr , NonterminalKind :: Expr (Expr) , | parser : & mut Parser <'b >| parser . parse_expr () , | x : ptr :: P < ast :: Expr >| Some (x)) ; parse_macro_arg ! (Ty , NonterminalKind :: Ty , | parser : & mut Parser <'b >| parser . parse_ty () , | x : ptr :: P < ast :: Ty >| Some (x)) ; parse_macro_arg ! (Pat , NonterminalKind :: Pat (PatParam { inferred : false }) , | parser : & mut Parser <'b >| parser . parse_pat_no_top_alt (None , None) , | x : ptr :: P < ast :: Pat >| Some (x)) ; parse_macro_arg ! (Item , NonterminalKind :: Item , | parser : & mut Parser <'b >| parser . parse_item (ForceCollect :: No) , | x : Option < ptr :: P < ast :: Item >>| x) ; None }
};
}
