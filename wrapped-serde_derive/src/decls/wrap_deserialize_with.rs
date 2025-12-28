macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! wrap_deserialize_with {
    () => {
        deps!();
        # [doc = " This function wraps the expression in `#[serde(deserialize_with = \"...\")]`"] # [doc = " in a trait to prevent it from accessing the internal `Deserialize` state."] fn wrap_deserialize_with (params : & Parameters , value_ty : & TokenStream , deserialize_with : & syn :: ExprPath ,) -> (TokenStream , TokenStream) { let this_type = & params . this_type ; let (de_impl_generics , de_ty_generics , ty_generics , where_clause) = params . generics_with_de_lifetime () ; let delife = params . borrowed . de_lifetime () ; let deserializer_var = quote ! (__deserializer) ; let value = quote_spanned ! { deserialize_with . span () => # deserialize_with (# deserializer_var) ? } ; let wrapper = quote ! { # [doc (hidden)] struct __DeserializeWith # de_impl_generics # where_clause { value : # value_ty , phantom : _serde ::# private :: PhantomData <# this_type # ty_generics >, lifetime : _serde ::# private :: PhantomData <&# delife () >, } # [automatically_derived] impl # de_impl_generics _serde :: Deserialize <# delife > for __DeserializeWith # de_ty_generics # where_clause { fn deserialize < __D > (# deserializer_var : __D) -> _serde ::# private :: Result < Self , __D :: Error > where __D : _serde :: Deserializer <# delife >, { _serde ::# private :: Ok (__DeserializeWith { value : # value , phantom : _serde ::# private :: PhantomData , lifetime : _serde ::# private :: PhantomData , }) } } } ; let wrapper_ty = quote ! (__DeserializeWith # de_ty_generics) ; (wrapper , wrapper_ty) }
    };
}

wrap_deserialize_with!();