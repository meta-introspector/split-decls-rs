// Generated macro for impl_71 (impl)
macro_rules! Depcrate_arrayvecimpl_71 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_71"}
// Dependencies: {}
impl < A : Array > LowerExp for ArrayVec < A > where A :: Item : LowerExp , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } LowerExp :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
