macro_rules! deps {
    () => {
        TokenMarker!();
    };
}

macro_rules! macro_448 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] pub_if_not_doc ! { # [doc (hidden)] # [allow (non_snake_case)] pub fn Lit (marker : lookahead :: TokenMarker) -> Lit { match marker { } } }
    };
}

macro_448!()