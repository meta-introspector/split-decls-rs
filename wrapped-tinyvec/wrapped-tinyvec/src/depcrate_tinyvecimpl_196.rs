// Generated macro for impl_196 (impl)
macro_rules! Depcrate_tinyvecimpl_196 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_196"}
// Dependencies: {}
impl < A : Array > Binary for TinyVec < A > where A :: Item : Binary , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Binary :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
