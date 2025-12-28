macro_rules! deps {
    () => {
        DataExt!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl DataExt for DataStruct { fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > { map_fields (& self . fields) } fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > { vec ! [self . fields ()] } fn tag (& self) -> Option < Ident > { None } }
    };
}

impl_3!()