// Generated macro for impl_73 (impl)
macro_rules! Depcrate_arrayvecimpl_73 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_73"}
// Dependencies: {}
impl < A : Array > Octal for ArrayVec < A > where A :: Item : Octal , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Octal :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
