macro_rules! deps {
    () => {
        LitRepr!();
    };
}

macro_rules! macro_412 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A byte string literal: `b\"foo\"`."] pub struct LitByteStr { repr : Box < LitRepr >, } }
    };
}

macro_412!()