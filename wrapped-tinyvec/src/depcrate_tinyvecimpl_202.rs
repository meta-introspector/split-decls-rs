// Generated macro for impl_202 (impl)
macro_rules! Depcrate_tinyvecimpl_202 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_202"}
// Dependencies: {}
impl < A : Array > Pointer for TinyVec < A > where A :: Item : Pointer , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Pointer :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
