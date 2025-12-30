// Generated macro for impl_123 (impl)
macro_rules! Depcrate_slicevecimpl_123 {
() => {
// Module: crate::slicevec
// Provides: {"impl_123"}
// Dependencies: {}
impl < 's , T > Binary for SliceVec < 's , T > where T : Binary , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Binary :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
