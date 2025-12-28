macro_rules! require_db_lifetime {
    () => {
        # [doc = " Require that either there is exactly one lifetime parameter."] pub (crate) fn require_db_lifetime (generics : & syn :: Generics) -> syn :: Result < () > { if generics . params . is_empty () { return Err (syn :: Error :: new_spanned (generics , "this definition must have a `'db` lifetime" ,)) ; } for (param , index) in generics . params . iter () . zip (0 ..) { let error = match param { syn :: GenericParam :: Lifetime (_) => index > 0 , syn :: GenericParam :: Type (_) | syn :: GenericParam :: Const (_) => true , } ; if error { return Err (syn :: Error :: new_spanned (param , "only a single lifetime parameter is accepted" ,)) ; } } Ok (()) }
    };
}

require_db_lifetime!();