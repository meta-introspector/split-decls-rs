// Generated macro for db_lifetime (function)
macro_rules! Depcrate_db_lifetimedb_lifetime {
() => {
// Module: crate::db_lifetime
// Provides: {"db_lifetime"}
// Dependencies: {}
# [doc = " Return the `'db` lifetime given by the user, or a default."] # [doc = " The generics ought to have been checked with `require_db_lifetime` already."] pub (crate) fn db_lifetime (generics : & syn :: Generics) -> syn :: Lifetime { if let Some (lt) = generics . lifetimes () . next () { lt . lifetime . clone () } else { default_db_lifetime (generics . span ()) } }
};
}
