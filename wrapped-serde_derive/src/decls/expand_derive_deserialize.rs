macro_rules! deps {
    () => {
        Stmts!();
        Derive!();
        Container!();
        Ctxt!();
        Parameters!();
    };
}

macro_rules! expand_derive_deserialize {
    () => {
        deps!();
        pub fn expand_derive_deserialize (input : & mut syn :: DeriveInput) -> syn :: Result < TokenStream > { replace_receiver (input) ; let ctxt = Ctxt :: new () ; let Some (cont) = Container :: from_ast (& ctxt , input , Derive :: Deserialize , & private . ident ()) else { return Err (ctxt . check () . unwrap_err ()) ; } ; precondition (& ctxt , & cont) ; ctxt . check () ? ; let ident = & cont . ident ; let params = Parameters :: new (& cont) ; let (de_impl_generics , _ , ty_generics , where_clause) = params . generics_with_de_lifetime () ; let body = Stmts (deserialize_body (& cont , & params)) ; let delife = params . borrowed . de_lifetime () ; let allow_deprecated = allow_deprecated (input) ; let impl_block = if let Some (remote) = cont . attrs . remote () { let vis = & input . vis ; let used = pretend :: pretend_used (& cont , params . is_packed) ; quote ! { # [automatically_derived] # allow_deprecated impl # de_impl_generics # ident # ty_generics # where_clause { # vis fn deserialize < __D > (__deserializer : __D) -> _serde ::# private :: Result <# remote # ty_generics , __D :: Error > where __D : _serde :: Deserializer <# delife >, { # used # body } } } } else { let fn_deserialize_in_place = deserialize_in_place_body (& cont , & params) ; quote ! { # [automatically_derived] # allow_deprecated impl # de_impl_generics _serde :: Deserialize <# delife > for # ident # ty_generics # where_clause { fn deserialize < __D > (__deserializer : __D) -> _serde ::# private :: Result < Self , __D :: Error > where __D : _serde :: Deserializer <# delife >, { # body } # fn_deserialize_in_place } } } ; Ok (dummy :: wrap_in_const (cont . attrs . custom_serde_path () , impl_block ,)) }
    };
}

expand_derive_deserialize!();