macro_rules! deps {
    () => {
        Container!();
        DeTypeGenerics!();
        Parameters!();
        DeImplGenerics!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl Parameters { fn new (cont : & Container) -> Self { let local = cont . ident . clone () ; let this_type = this :: this_type (cont) ; let this_value = this :: this_value (cont) ; let borrowed = borrowed_lifetimes (cont) ; let generics = build_generics (cont , & borrowed) ; let has_getter = cont . data . has_getter () ; let is_packed = cont . attrs . is_packed () ; Parameters { local , this_type , this_value , generics , borrowed , has_getter , is_packed , } } # [doc = " Type name to use in error messages and `&'static str` arguments to"] # [doc = " various Deserializer methods."] fn type_name (& self) -> String { self . this_type . segments . last () . unwrap () . ident . to_string () } # [doc = " Split the data structure's generics into the pieces to use for its"] # [doc = " `Deserialize` impl, augmented with an additional `'de` lifetime for use"] # [doc = " as the `Deserialize` trait's lifetime."] fn generics_with_de_lifetime (& self ,) -> (DeImplGenerics , DeTypeGenerics , syn :: TypeGenerics , Option < & syn :: WhereClause > ,) { let de_impl_generics = DeImplGenerics (self) ; let de_ty_generics = DeTypeGenerics (self) ; let (_ , ty_generics , where_clause) = self . generics . split_for_impl () ; (de_impl_generics , de_ty_generics , ty_generics , where_clause) } }
    };
}

impl_189!()