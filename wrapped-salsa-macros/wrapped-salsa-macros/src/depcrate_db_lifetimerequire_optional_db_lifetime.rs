// Generated macro for require_optional_db_lifetime (function)
macro_rules! Depcrate_db_lifetimerequire_optional_db_lifetime {
() => {
// Module: crate::db_lifetime
// Provides: {"require_optional_db_lifetime"}
// Dependencies: {}
# [doc = " Require that either there are no generics or exactly one lifetime parameter."] pub (crate) fn require_optional_db_lifetime (generics : & syn :: Generics) -> syn :: Result < () > { if generics . params . is_empty () { return Ok (()) ; } require_db_lifetime (generics) ? ; Ok (()) }
};
}
