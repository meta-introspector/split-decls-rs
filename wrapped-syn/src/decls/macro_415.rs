macro_rules! deps {
    () => {
        LitRepr!();
    };
}

macro_rules! macro_415 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A character literal: `'a'`."] pub struct LitChar { repr : Box < LitRepr >, } }
    };
}

macro_415!()