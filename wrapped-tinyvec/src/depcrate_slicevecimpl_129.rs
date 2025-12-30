// Generated macro for impl_129 (impl)
macro_rules! Depcrate_slicevecimpl_129 {
() => {
// Module: crate::slicevec
// Provides: {"impl_129"}
// Dependencies: {}
impl < 's , T > Pointer for SliceVec < 's , T > where T : Pointer , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Pointer :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
