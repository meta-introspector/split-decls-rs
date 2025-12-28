macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! wrap_serialize_with {
    () => {
        deps!();
        fn wrap_serialize_with (params : & Parameters , serialize_with : & syn :: ExprPath , field_tys : & [& syn :: Type] , field_exprs : & [TokenStream] ,) -> TokenStream { let this_type = & params . this_type ; let (_ , ty_generics , where_clause) = params . generics . split_for_impl () ; let wrapper_generics = if field_exprs . is_empty () { params . generics . clone () } else { bound :: with_lifetime_bound (& params . generics , "'__a") } ; let (wrapper_impl_generics , wrapper_ty_generics , _) = wrapper_generics . split_for_impl () ; let field_access = (0 .. field_exprs . len ()) . map (| n | { Member :: Unnamed (Index { index : n as u32 , span : Span :: call_site () , }) }) ; let self_var = quote ! (self) ; let serializer_var = quote ! (__s) ; let wrapper_serialize = quote_spanned ! { serialize_with . span () => # serialize_with (# (# self_var . values .# field_access ,) * # serializer_var) } ; quote ! (& { # [doc (hidden)] struct __SerializeWith # wrapper_impl_generics # where_clause { values : (# (&'__a # field_tys ,) *) , phantom : _serde ::# private :: PhantomData <# this_type # ty_generics >, } # [automatically_derived] impl # wrapper_impl_generics _serde :: Serialize for __SerializeWith # wrapper_ty_generics # where_clause { fn serialize < __S > (&# self_var , # serializer_var : __S) -> _serde ::# private :: Result < __S :: Ok , __S :: Error > where __S : _serde :: Serializer , { # wrapper_serialize } } __SerializeWith { values : (# (# field_exprs ,) *) , phantom : _serde ::# private :: PhantomData ::<# this_type # ty_generics >, } }) }
    };
}

wrap_serialize_with!();