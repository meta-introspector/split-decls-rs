// Generated macro for imp (module)
macro_rules! Depcrate_fmtimp {
() => {
// Module: crate::fmt
// Provides: {"imp"}
// Dependencies: {}
# [doc = " Implementation details."] # [doc = ""] # [doc = " You don't need these unless you are implementing"] # [doc = " a new format."] pub mod imp { use std :: { iter , slice , mem } ; use std :: default :: Default ; # [doc = " Describes how to fix up encodings when concatenating."] # [doc = ""] # [doc = " We can drop characters on either side of the splice,"] # [doc = " and insert up to 4 bytes in the middle."] pub struct Fixup { pub drop_left : u32 , pub drop_right : u32 , pub insert_len : u32 , pub insert_bytes : [u8 ; 4] , } impl Default for Fixup { # [inline (always)] fn default () -> Fixup { Fixup { drop_left : 0 , drop_right : 0 , insert_len : 0 , insert_bytes : [0 ; 4] , } } } # [inline (always)] unsafe fn from_u32_unchecked (n : u32) -> char { mem :: transmute (n) } pub struct SingleByteCharIndices < 'a > { inner : iter :: Enumerate < slice :: Iter < 'a , u8 > > , } impl < 'a > Iterator for SingleByteCharIndices < 'a > { type Item = (usize , char) ; # [inline] fn next (& mut self) -> Option < (usize , char) > { self . inner . next () . map (| (i , & b) | unsafe { (i , from_u32_unchecked (b as u32)) }) } } impl < 'a > SingleByteCharIndices < 'a > { # [inline] pub fn new (buf : & 'a [u8]) -> SingleByteCharIndices < 'a > { SingleByteCharIndices { inner : buf . iter () . enumerate () , } } } }
};
}
