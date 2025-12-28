macro_rules! deps {
    () => {
        SubdiagnosticDeriveVariantBuilder!();
        HasFieldMap!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'parent , 'a > HasFieldMap for SubdiagnosticDeriveVariantBuilder < 'parent , 'a > { fn get_field_binding (& self , field : & String) -> Option < & TokenStream > { self . fields . get (field) } }
    };
}

impl_31!()