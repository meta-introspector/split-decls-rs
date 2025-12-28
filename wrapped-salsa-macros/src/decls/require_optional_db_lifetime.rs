macro_rules! require_optional_db_lifetime {
    () => {
        # [doc = " Require that either there are no generics or exactly one lifetime parameter."] pub (crate) fn require_optional_db_lifetime (generics : & syn :: Generics) -> syn :: Result < () > { if generics . params . is_empty () { return Ok (()) ; } require_db_lifetime (generics) ? ; Ok (()) }
    };
}

require_optional_db_lifetime!();