macro_rules! deps {
    () => {
        LitRepr!();
    };
}

macro_rules! macro_411 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A UTF-8 string literal: `\"foo\"`."] pub struct LitStr { repr : Box < LitRepr >, } }
    };
}

macro_411!()