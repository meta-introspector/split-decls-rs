// Generated macro for impl_attribute (function)
macro_rules! Depcrate_attributeimpl_attribute {
() => {
// Module: crate::attribute
// Provides: {"impl_attribute"}
// Dependencies: {}
fn impl_attribute (input : syn :: ItemFn , args : TokenStream , is_test : bool ,) -> syn :: Result < TokenStream > { if ! input . sig . inputs . is_empty () { let msg = "Cannot accept arguments" ; return Err (syn :: Error :: new_spanned (& input . sig . ident , msg)) ; } let args = AttributeArgs :: parse_terminated . parse (args) ? ; if let Some (_async) = input . sig . asyncness { # [cfg (not (feature = "sync"))] return Err (syn :: Error :: new_spanned (_async , "feature `sync` required for async functions" ,)) ; # [cfg (feature = "sync")] { let path = tokio_attribute_path (is_test) ; if ! input . attrs . iter () . any (| attr | attr . path == path) { let msg = if is_test { "Attribute must be succeeded by #[tokio::test] for async tests" } else { "Attribute must be succeeded by #[tokio::main] for async functions" } ; return Err (syn :: Error :: new_spanned (args , msg)) ; } impl_async (Config :: parse (args , is_test) ? , input) } } else { impl_sync (Config :: parse (args , is_test) ? , input) } }
};
}
