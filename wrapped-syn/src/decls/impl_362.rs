macro_rules! impl_362 {
    () => {
        impl From < ItemStruct > for DeriveInput { fn from (input : ItemStruct) -> DeriveInput { DeriveInput { attrs : input . attrs , vis : input . vis , ident : input . ident , generics : input . generics , data : Data :: Struct (DataStruct { struct_token : input . struct_token , fields : input . fields , semi_token : input . semi_token , }) , } } }
    };
}

impl_362!()