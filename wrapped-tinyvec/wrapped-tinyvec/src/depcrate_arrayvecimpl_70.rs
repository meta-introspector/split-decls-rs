// Generated macro for impl_70 (impl)
macro_rules! Depcrate_arrayvecimpl_70 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_70"}
// Dependencies: {}
impl < A : Array > Display for ArrayVec < A > where A :: Item : Display , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Display :: fmt (elem , f) ? ; } if f . alternate () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
