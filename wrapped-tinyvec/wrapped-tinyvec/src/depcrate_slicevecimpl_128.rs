// Generated macro for impl_128 (impl)
macro_rules! Depcrate_slicevecimpl_128 {
() => {
// Module: crate::slicevec
// Provides: {"impl_128"}
// Dependencies: {}
impl < 's , T > Octal for SliceVec < 's , T > where T : Octal , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Octal :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
