macro_rules! deps {
    () => {
        FieldConfig!();
    };
}

macro_rules! get_field_config {
    () => {
        deps!();
        fn get_field_config (attrs : & [syn :: Attribute]) -> syn :: Result < FieldConfig > { mod kw { syn :: custom_keyword ! (skip) ; syn :: custom_keyword ! (with) ; } enum Input { Skip { _skip : kw :: skip , } , With { _with : kw :: with , _eq : syn :: Token ! [=] , path : syn :: Path , } , } impl syn :: parse :: Parse for Input { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { let lookahead = input . lookahead1 () ; if lookahead . peek (kw :: skip) { Ok (Self :: Skip { _skip : input . parse () ? , }) } else if lookahead . peek (kw :: with) { Ok (Self :: With { _with : input . parse () ? , _eq : input . parse () ? , path : input . parse () ? , }) } else { Err (lookahead . error ()) } } } for attr in attrs { let syn :: Meta :: List (meta) = & attr . meta else { continue ; } ; let Some (path) = meta . path . get_ident () else { continue ; } ; if path != "typesize" { continue ; } let input = syn :: parse :: < Input > (meta . tokens . clone () . into ()) ? ; return Ok (match input { Input :: Skip { .. } => FieldConfig :: Skip , Input :: With { path , .. } => FieldConfig :: With (path) , }) ; } Ok (FieldConfig :: Default) }
    };
}

get_field_config!()