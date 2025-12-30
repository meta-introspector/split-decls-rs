// Generated macro for impl_200 (impl)
macro_rules! Depcrate_tagimpl_200 {
() => {
// Module: crate::tag
// Provides: {"impl_200"}
// Dependencies: {}
impl Tag { # [doc = " Interprets the [`Tag`] as an integer."] # [inline] pub (super) const fn value (self) -> usize { match self { Self :: None => 0 , Self :: First => 1 , Self :: Second => 2 , Self :: Both => 3 , } } # [doc = " Returns the tag embedded in the pointer."] # [inline] pub (super) fn into_tag < P > (ptr : * const P) -> Self { match ptr . addr () & 3 { 0 => Tag :: None , 1 => Tag :: First , 2 => Tag :: Second , _ => Tag :: Both , } } # [doc = " Sets a tag, overwriting any existing tag in the pointer."] # [inline] pub (super) fn update_tag < P > (ptr : * const P , tag : Tag) -> * const P { ptr . map_addr (| addr | (addr & (! 3)) | tag . value ()) } # [doc = " Returns the pointer with the tag bits erased."] # [inline] pub (super) fn unset_tag < P > (ptr : * const P) -> * const P { ptr . map_addr (| addr | addr & (! 3)) } }
};
}
