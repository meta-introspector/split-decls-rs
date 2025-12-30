// Generated macro for convert_primitive (macro)
macro_rules! Depcrate_implsconvert_primitive {
() => {
// Module: crate::impls
// Provides: {"convert_primitive"}
// Dependencies: {}
macro_rules ! convert_primitive { ($ ($ t : ty : $ from : ident , $ to : ident ,) *) => { $ (impl <'v > From <$ t > for ValueBag <'v > { # [inline] fn from (v : $ t) -> Self { ValueBag ::$ from (v) } } impl <'a , 'v > From <&'a $ t > for ValueBag <'v > { # [inline] fn from (v : &'a $ t) -> Self { ValueBag ::$ from (* v) } } impl <'v > From < Option <$ t >> for ValueBag <'v > { # [inline] fn from (v : Option <$ t >) -> Self { ValueBag :: from_option (v) } } impl <'v > TryFrom < ValueBag <'v >> for $ t { type Error = Error ; # [inline] fn try_from (v : ValueBag <'v >) -> Result < Self , Error > { v .$ to () . ok_or_else (|| Error :: msg ("conversion failed")) ? . try_into () . map_err (| _ | Error :: msg ("conversion failed")) } }) * } ; }
};
}
