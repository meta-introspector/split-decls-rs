// Generated macro for impl_204 (impl)
macro_rules! Depcrate_tinyvecimpl_204 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_204"}
// Dependencies: {}
impl < A : Array > UpperHex for TinyVec < A > where A :: Item : UpperHex , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } UpperHex :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
