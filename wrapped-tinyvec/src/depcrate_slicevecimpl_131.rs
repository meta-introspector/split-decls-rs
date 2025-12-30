// Generated macro for impl_131 (impl)
macro_rules! Depcrate_slicevecimpl_131 {
() => {
// Module: crate::slicevec
// Provides: {"impl_131"}
// Dependencies: {}
impl < 's , T > UpperHex for SliceVec < 's , T > where T : UpperHex , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } UpperHex :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
