macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < A : Array > Binary for TinyVec < A > where A :: Item : Binary , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Binary :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
    };
}

impl_171!()