// Generated macro for impl_76 (impl)
macro_rules! Depcrate_arrayvecimpl_76 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_76"}
// Dependencies: {}
impl < A : Array > UpperHex for ArrayVec < A > where A :: Item : UpperHex , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } UpperHex :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
