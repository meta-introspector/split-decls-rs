// Generated macro for require_no_generics (function)
macro_rules! Depcrate_db_lifetimerequire_no_generics {
() => {
// Module: crate::db_lifetime
// Provides: {"require_no_generics"}
// Dependencies: {}
pub (crate) fn require_no_generics (generics : & syn :: Generics) -> syn :: Result < () > { if let Some (param) = generics . params . iter () . next () { return Err (syn :: Error :: new_spanned (param , "generic parameters not allowed here" ,)) ; } Ok (()) }
};
}
