macro_rules! deps {
    () => {
        SalsaField!();
    };
}

macro_rules! FIELD_OPTION_ATTRIBUTES {
    () => {
        deps!();
        # [allow (clippy :: type_complexity)] pub (crate) const FIELD_OPTION_ATTRIBUTES : & [(& str , fn (& syn :: Attribute , & mut SalsaField) -> syn :: Result < () > ,)] = & [("tracked" , | _ , ef | { ef . has_tracked_attr = true ; Ok (()) }) , ("default" , | _ , ef | { ef . has_default_attr = true ; Ok (()) }) , ("returns" , | attr , ef | { ef . returns = attr . parse_args_with (syn :: Ident :: parse_any) ? ; Ok (()) }) , ("no_eq" , | _ , ef | { ef . has_no_eq_attr = true ; Ok (()) }) , ("get" , | attr , ef | { ef . get_name = attr . parse_args () ? ; Ok (()) }) , ("set" , | attr , ef | { ef . set_name = attr . parse_args () ? ; Ok (()) }) , ("maybe_update" , | attr , ef | { ef . maybe_update_attr = Some (attr . parse_args_with (| parser : ParseStream | { let expr = parser . parse :: < syn :: Expr > () ? ; Ok ((attr . path () . clone () , expr)) }) ?) ; Ok (()) }) ,] ;
    };
}

FIELD_OPTION_ATTRIBUTES!()