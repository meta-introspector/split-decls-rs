// Generated macro for impl_72 (impl)
macro_rules! Depcrate_arrayvecimpl_72 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_72"}
// Dependencies: {}
impl < A : Array > LowerHex for ArrayVec < A > where A :: Item : LowerHex , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } LowerHex :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
