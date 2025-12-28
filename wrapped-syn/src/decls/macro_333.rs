macro_rules! deps {
    () => {
        TokenMarker!();
    };
}

macro_rules! macro_333 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] pub_if_not_doc ! { # [doc (hidden)] # [allow (non_snake_case)] pub fn Ident (marker : lookahead :: TokenMarker) -> Ident { match marker { } } }
    };
}

macro_333!()