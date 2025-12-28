macro_rules! deps {
    () => {
        Parameters!();
        Variant!();
        Style!();
    };
}

macro_rules! unwrap_to_variant_closure {
    () => {
        deps!();
        fn unwrap_to_variant_closure (params : & Parameters , variant : & Variant , with_wrapper : bool ,) -> TokenStream { let this_value = & params . this_value ; let variant_ident = & variant . ident ; let (arg , wrapper) = if with_wrapper { (quote ! { __wrap } , quote ! { __wrap . value }) } else { let field_tys = variant . fields . iter () . map (| field | field . ty) ; (quote ! { __wrap : (# (# field_tys) ,*) } , quote ! { __wrap }) } ; let field_access = (0 .. variant . fields . len ()) . map (| n | { Member :: Unnamed (Index { index : n as u32 , span : Span :: call_site () , }) }) ; match variant . style { Style :: Struct if variant . fields . len () == 1 => { let member = & variant . fields [0] . member ; quote ! { |# arg | # this_value ::# variant_ident { # member : # wrapper } } } Style :: Struct => { let members = variant . fields . iter () . map (| field | & field . member) ; quote ! { |# arg | # this_value ::# variant_ident { # (# members : # wrapper .# field_access) ,* } } } Style :: Tuple => quote ! { |# arg | # this_value ::# variant_ident (# (# wrapper .# field_access) ,*) } , Style :: Newtype => quote ! { |# arg | # this_value ::# variant_ident (# wrapper) } , Style :: Unit => quote ! { |# arg | # this_value ::# variant_ident } , } }
    };
}

unwrap_to_variant_closure!()