macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < A : Array > LowerHex for TinyVec < A > where A :: Item : LowerHex , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } LowerHex :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
    };
}

impl_175!()