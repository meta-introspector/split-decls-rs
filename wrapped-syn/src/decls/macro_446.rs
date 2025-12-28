macro_rules! deps {
    () => {
        TokenMarker!();
    };
}

macro_rules! macro_446 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] pub_if_not_doc ! { # [doc (hidden)] # [allow (non_snake_case)] pub fn LitBool (marker : lookahead :: TokenMarker) -> LitBool { match marker { } } }
    };
}

macro_446!()