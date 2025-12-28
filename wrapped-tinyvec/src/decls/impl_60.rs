macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < A : Array > Debug for ArrayVec < A > where A :: Item : Debug , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () && ! self . is_empty () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Debug :: fmt (elem , f) ? ; } if f . alternate () && ! self . is_empty () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
    };
}

impl_60!()