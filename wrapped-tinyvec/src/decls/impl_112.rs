macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 's , T > Octal for SliceVec < 's , T > where T : Octal , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Octal :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
    };
}

impl_112!()