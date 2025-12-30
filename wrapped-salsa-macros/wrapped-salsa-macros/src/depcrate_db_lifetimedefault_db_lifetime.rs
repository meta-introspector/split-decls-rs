// Generated macro for default_db_lifetime (function)
macro_rules! Depcrate_db_lifetimedefault_db_lifetime {
() => {
// Module: crate::db_lifetime
// Provides: {"default_db_lifetime"}
// Dependencies: {}
# [doc = " Normally we try to use whatever lifetime parameter the user gave us"] # [doc = " to represent `'db`; but if they didn't give us one, we need to use a default"] # [doc = " name. We choose `'db`."] pub (crate) fn default_db_lifetime (span : Span) -> syn :: Lifetime { syn :: Lifetime { apostrophe : span , ident : syn :: Ident :: new ("db" , span) , } }
};
}
