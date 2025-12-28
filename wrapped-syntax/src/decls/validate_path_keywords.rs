macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_path_keywords {
    () => {
        deps!();
        fn validate_path_keywords (segment : ast :: PathSegment , errors : & mut Vec < SyntaxError >) { let path = segment . parent_path () ; let is_path_start = segment . coloncolon_token () . is_none () && path . qualifier () . is_none () ; if let Some (token) = segment . self_token () { if ! is_path_start { errors . push (SyntaxError :: new ("The `self` keyword is only allowed as the first segment of a path" , token . text_range () ,)) ; } } else if let Some (token) = segment . crate_token () && (! is_path_start || use_prefix (path) . is_some ()) { errors . push (SyntaxError :: new ("The `crate` keyword is only allowed as the first segment of a path" , token . text_range () ,)) ; } fn use_prefix (mut path : ast :: Path) -> Option < ast :: Path > { for node in path . syntax () . ancestors () . skip (1) { match_ast ! { match node { ast :: UseTree (it) => if let Some (tree_path) = it . path () { if tree_path != path { return Some (tree_path) ; } } , ast :: UseTreeList (_) => continue , ast :: Path (parent) => path = parent , _ => return None , } } ; } None } }
    };
}

validate_path_keywords!();