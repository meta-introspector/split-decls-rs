macro_rules! deps {
    () => {
        LitIntRepr!();
    };
}

macro_rules! macro_417 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An integer literal: `1` or `1u16`."] pub struct LitInt { repr : Box < LitIntRepr >, } }
    };
}

macro_417!()