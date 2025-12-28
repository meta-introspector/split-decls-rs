macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 's , T > LowerExp for SliceVec < 's , T > where T : LowerExp , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } LowerExp :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
    };
}

impl_110!()