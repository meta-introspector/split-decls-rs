macro_rules! deps {
    () => {
        HasFieldMap!();
        DiagnosticDeriveVariantBuilder!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl HasFieldMap for DiagnosticDeriveVariantBuilder { fn get_field_binding (& self , field : & String) -> Option < & TokenStream > { self . field_map . get (field) } }
    };
}

impl_14!();