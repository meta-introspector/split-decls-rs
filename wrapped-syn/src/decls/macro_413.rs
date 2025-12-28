macro_rules! deps {
    () => {
        LitRepr!();
    };
}

macro_rules! macro_413 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A nul-terminated C-string literal: `c\"foo\"`."] pub struct LitCStr { repr : Box < LitRepr >, } }
    };
}

macro_413!()