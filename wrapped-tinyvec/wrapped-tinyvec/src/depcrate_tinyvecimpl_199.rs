// Generated macro for impl_199 (impl)
macro_rules! Depcrate_tinyvecimpl_199 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_199"}
// Dependencies: {}
impl < A : Array > LowerExp for TinyVec < A > where A :: Item : LowerExp , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } LowerExp :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
