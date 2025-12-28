macro_rules! deps {
    () => {
        Style!();
        Data!();
        Container!();
    };
}

macro_rules! pretend_variants_used {
    () => {
        deps!();
        fn pretend_variants_used (cont : & Container) -> TokenStream { let variants = match & cont . data { Data :: Enum (variants) => variants , Data :: Struct (_ , _) => { return quote ! () ; } } ; let type_ident = & cont . ident ; let (_ , ty_generics , _) = cont . generics . split_for_impl () ; let turbofish = ty_generics . as_turbofish () ; let cases = variants . iter () . map (| variant | { let variant_ident = & variant . ident ; let placeholders = & (0 .. variant . fields . len ()) . map (| i | format_ident ! ("__v{}" , i)) . collect :: < Vec < _ > > () ; let pat = match variant . style { Style :: Struct => { let members = variant . fields . iter () . map (| field | & field . member) ; quote ! ({ # (# members : # placeholders) ,* }) } Style :: Tuple | Style :: Newtype => quote ! ((# (# placeholders) ,*)) , Style :: Unit => quote ! () , } ; quote ! { match _serde ::# private :: None { _serde ::# private :: Some ((# (# placeholders ,) *)) => { let _ = # type_ident ::# variant_ident # turbofish # pat ; } _ => { } } } }) ; quote ! (# (# cases) *) }
    };
}

pretend_variants_used!();