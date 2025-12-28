macro_rules! deps {
    () => {
        Lifetime!();
        TokenMarker!();
    };
}

macro_rules! macro_406 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] pub_if_not_doc ! { # [doc (hidden)] # [allow (non_snake_case)] pub fn Lifetime (marker : lookahead :: TokenMarker) -> Lifetime { match marker { } } }
    };
}

macro_406!()