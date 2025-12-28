macro_rules! deps {
    () => {
        LitFloatRepr!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl From < Literal > for LitFloat { # [track_caller] fn from (token : Literal) -> Self { let repr = token . to_string () ; if let Some ((digits , suffix)) = value :: parse_lit_float (& repr) { LitFloat { repr : Box :: new (LitFloatRepr { token , digits , suffix , }) , } } else { panic ! ("not a float literal: `{}`" , repr) ; } } }
    };
}

impl_431!()