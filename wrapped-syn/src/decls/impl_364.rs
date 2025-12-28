macro_rules! impl_364 {
    () => {
        impl From < ItemUnion > for DeriveInput { fn from (input : ItemUnion) -> DeriveInput { DeriveInput { attrs : input . attrs , vis : input . vis , ident : input . ident , generics : input . generics , data : Data :: Union (DataUnion { union_token : input . union_token , fields : input . fields , }) , } } }
    };
}

impl_364!()