// Generated macro for impl_198 (impl)
macro_rules! Depcrate_tinyvecimpl_198 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_198"}
// Dependencies: {}
impl < A : Array > Display for TinyVec < A > where A :: Item : Display , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Display :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
