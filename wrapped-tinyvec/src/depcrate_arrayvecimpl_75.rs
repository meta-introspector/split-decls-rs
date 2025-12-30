// Generated macro for impl_75 (impl)
macro_rules! Depcrate_arrayvecimpl_75 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_75"}
// Dependencies: {}
impl < A : Array > UpperExp for ArrayVec < A > where A :: Item : UpperExp , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } UpperExp :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
