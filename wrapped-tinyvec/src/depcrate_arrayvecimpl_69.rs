// Generated macro for impl_69 (impl)
macro_rules! Depcrate_arrayvecimpl_69 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_69"}
// Dependencies: {}
impl < A : Array > Debug for ArrayVec < A > where A :: Item : Debug , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter) -> core :: fmt :: Result { write ! (f , "[") ? ; if f . alternate () && ! self . is_empty () { write ! (f , "\n    ") ? ; } for (i , elem) in self . iter () . enumerate () { if i > 0 { write ! (f , ",{}" , if f . alternate () { "\n    " } else { " " }) ? ; } Debug :: fmt (elem , f) ? ; } if f . alternate () && ! self . is_empty () { write ! (f , ",\n") ? ; } write ! (f , "]") } }
};
}
