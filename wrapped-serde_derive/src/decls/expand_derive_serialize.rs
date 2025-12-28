macro_rules! deps {
    () => {
        Stmts!();
        Container!();
        Derive!();
        Parameters!();
        Ctxt!();
    };
}

macro_rules! expand_derive_serialize {
    () => {
        deps!();
        pub fn expand_derive_serialize (input : & mut syn :: DeriveInput) -> syn :: Result < TokenStream > { replace_receiver (input) ; let ctxt = Ctxt :: new () ; let Some (cont) = Container :: from_ast (& ctxt , input , Derive :: Serialize , & private . ident ()) else { return Err (ctxt . check () . unwrap_err ()) ; } ; precondition (& ctxt , & cont) ; ctxt . check () ? ; let ident = & cont . ident ; let params = Parameters :: new (& cont) ; let (impl_generics , ty_generics , where_clause) = params . generics . split_for_impl () ; let body = Stmts (serialize_body (& cont , & params)) ; let allow_deprecated = allow_deprecated (input) ; let impl_block = if let Some (remote) = cont . attrs . remote () { let vis = & input . vis ; let used = pretend :: pretend_used (& cont , params . is_packed) ; quote ! { # [automatically_derived] # allow_deprecated impl # impl_generics # ident # ty_generics # where_clause { # vis fn serialize < __S > (__self : &# remote # ty_generics , __serializer : __S) -> _serde ::# private :: Result < __S :: Ok , __S :: Error > where __S : _serde :: Serializer , { # used # body } } } } else { quote ! { # [automatically_derived] # allow_deprecated impl # impl_generics _serde :: Serialize for # ident # ty_generics # where_clause { fn serialize < __S > (& self , __serializer : __S) -> _serde ::# private :: Result < __S :: Ok , __S :: Error > where __S : _serde :: Serializer , { # body } } } } ; Ok (dummy :: wrap_in_const (cont . attrs . custom_serde_path () , impl_block ,)) }
    };
}

expand_derive_serialize!()