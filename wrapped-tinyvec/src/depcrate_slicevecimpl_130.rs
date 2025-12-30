// Generated macro for impl_130 (impl)
macro_rules! Depcrate_slicevecimpl_130 {
() => {
// Module: crate::slicevec
// Provides: {"impl_130"}
// Dependencies: {}
impl < 's , T > UpperExp for SliceVec < 's , T > where T : UpperExp , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } UpperExp :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
