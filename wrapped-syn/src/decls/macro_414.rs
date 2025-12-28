macro_rules! deps {
    () => {
        LitRepr!();
    };
}

macro_rules! macro_414 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A byte literal: `b'f'`."] pub struct LitByte { repr : Box < LitRepr >, } }
    };
}

macro_414!()