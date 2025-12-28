macro_rules! deps {
    () => {
        Ctxt!();
        Symbol!();
    };
}

macro_rules! parse_lit_into_ty {
    () => {
        deps!();
        fn parse_lit_into_ty (cx : & Ctxt , attr_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < Option < syn :: Type > > { let Some (string) = get_lit_str (cx , attr_name , meta) ? else { return Ok (None) ; } ; Ok (match string . parse () { Ok (ty) => Some (ty) , Err (_) => { cx . error_spanned_by (& string , format ! ("failed to parse type: {} = {:?}" , attr_name , string . value ()) ,) ; None } }) }
    };
}

parse_lit_into_ty!();