macro_rules! deps {
    () => {
        Attributes!();
    };
}

macro_rules! parse_attributes {
    () => {
        deps!();
        fn parse_attributes (field : & syn :: Field) -> Attributes { let mut attrs = Attributes { ignore : false , project : None } ; for attr in & field . attrs { let meta = & attr . meta ; if ! meta . path () . is_ident ("stable_hasher") { continue ; } let mut any_attr = false ; let _ = attr . parse_nested_meta (| nested | { if nested . path . is_ident ("ignore") { attrs . ignore = true ; any_attr = true ; } if nested . path . is_ident ("project") { let _ = nested . parse_nested_meta (| meta | { if attrs . project . is_none () { attrs . project = meta . path . get_ident () . cloned () ; } any_attr = true ; Ok (()) }) ; } Ok (()) }) ; if ! any_attr { panic ! ("error parsing stable_hasher") ; } } attrs }
    };
}

parse_attributes!();