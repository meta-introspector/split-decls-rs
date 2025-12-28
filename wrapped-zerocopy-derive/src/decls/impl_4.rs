macro_rules! deps {
    () => {
        DataExt!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl DataExt for DataEnum { fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > { map_fields (self . variants . iter () . flat_map (| var | & var . fields)) } fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > { self . variants . iter () . map (| var | map_fields (& var . fields)) . collect () } fn tag (& self) -> Option < Ident > { Some (Ident :: new ("___ZerocopyTag" , Span :: call_site ())) } }
    };
}

impl_4!();