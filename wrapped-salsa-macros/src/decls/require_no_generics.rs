macro_rules! require_no_generics {
    () => {
        pub (crate) fn require_no_generics (generics : & syn :: Generics) -> syn :: Result < () > { if let Some (param) = generics . params . iter () . next () { return Err (syn :: Error :: new_spanned (param , "generic parameters not allowed here" ,)) ; } Ok (()) }
    };
}

require_no_generics!()