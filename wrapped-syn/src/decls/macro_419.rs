macro_rules! deps {
    () => {
        LitFloatRepr!();
    };
}

macro_rules! macro_419 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A floating point literal: `1f64` or `1.0e10f64`."] # [doc = ""] # [doc = " Must be finite. May not be infinite or NaN."] pub struct LitFloat { repr : Box < LitFloatRepr >, } }
    };
}

macro_419!()