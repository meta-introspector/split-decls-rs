// Generated macro for rewrite_reorderable_item (function)
macro_rules! Depcrate_reorderrewrite_reorderable_item {
() => {
// Module: crate::reorder
// Provides: {"rewrite_reorderable_item"}
// Dependencies: {}
fn rewrite_reorderable_item (context : & RewriteContext < '_ > , item : & ast :: Item , shape : Shape ,) -> RewriteResult { match item . kind { ast :: ItemKind :: ExternCrate (..) => rewrite_extern_crate (context , item , shape) , ast :: ItemKind :: Mod (_ , ident , _) => rewrite_mod (context , item , ident , shape) , _ => Err (RewriteError :: Unknown) , } }
};
}
