// Generated macro for rewrite_generics (function)
macro_rules! Depcrate_itemsrewrite_generics {
() => {
// Module: crate::items
// Provides: {"rewrite_generics"}
// Dependencies: {}
fn rewrite_generics (context : & RewriteContext < '_ > , ident : & str , generics : & ast :: Generics , shape : Shape ,) -> RewriteResult { if generics . params . is_empty () { return Ok (ident . to_owned ()) ; } let params = generics . params . iter () ; overflow :: rewrite_with_angle_brackets (context , ident , params , shape , generics . span) }
};
}
