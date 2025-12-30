// Generated macro for parse_attrs (function)
macro_rules! Depcrate_attrparse_attrs {
() => {
// Module: crate::attr
// Provides: {"parse_attrs"}
// Dependencies: {}
pub (crate) fn parse_attrs (cx : & Context , attrs : & [syn :: Attribute] , pos : Position) -> Attrs { let mut rename = None ; let mut transparent = None ; let mut skip = None ; let attrs = filter_attrs (cx , attrs , pos) ; for (def , meta) in & attrs { macro_rules ! lit_str { ($ field : ident) => { { let m = match meta { Meta :: NameValue (m) => m , _ => unreachable ! () , } ; let lit = match & m . value { syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (l) , .. }) => l , l => { cx . error (format_err ! (l , "expected string literal")) ; continue ; } } ; $ field = Some ((m . clone () , lit . clone ())) ; } } ; } if def . late_check (cx , & attrs) { continue ; } match def . name { "rename" => lit_str ! (rename) , "transparent" => transparent = Some (meta . span ()) , "skip" => skip = Some (meta . span ()) , _ => unreachable ! ("{}" , def . name) , } } Attrs { rename , transparent , skip , } }
};
}
