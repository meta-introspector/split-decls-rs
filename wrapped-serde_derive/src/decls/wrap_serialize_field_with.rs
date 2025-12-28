macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! wrap_serialize_field_with {
    () => {
        deps!();
        fn wrap_serialize_field_with (params : & Parameters , field_ty : & syn :: Type , serialize_with : & syn :: ExprPath , field_expr : & TokenStream ,) -> TokenStream { wrap_serialize_with (params , serialize_with , & [field_ty] , & [quote ! (# field_expr)]) }
    };
}

wrap_serialize_field_with!();