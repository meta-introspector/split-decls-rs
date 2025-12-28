macro_rules! deps {
    () => {
        DataExt!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl DataExt for DataUnion { fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > { map_fields (& self . fields . named) } fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > { vec ! [self . fields ()] } fn tag (& self) -> Option < Ident > { None } }
    };
}

impl_5!();