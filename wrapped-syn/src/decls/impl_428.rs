macro_rules! deps {
    () => {
        LitIntRepr!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl From < Literal > for LitInt { # [track_caller] fn from (token : Literal) -> Self { let repr = token . to_string () ; if let Some ((digits , suffix)) = value :: parse_lit_int (& repr) { LitInt { repr : Box :: new (LitIntRepr { token , digits , suffix , }) , } } else { panic ! ("not an integer literal: `{}`" , repr) ; } } }
    };
}

impl_428!();