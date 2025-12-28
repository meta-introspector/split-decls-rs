macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! wrap_deserialize_field_with {
    () => {
        deps!();
        fn wrap_deserialize_field_with (params : & Parameters , field_ty : & syn :: Type , deserialize_with : & syn :: ExprPath ,) -> (TokenStream , TokenStream) { wrap_deserialize_with (params , & quote ! (# field_ty) , deserialize_with) }
    };
}

wrap_deserialize_field_with!()