// Generated macro for impl_72 (impl)
macro_rules! Depcrate_tagimpl_72 {
() => {
// Module: crate::tag
// Provides: {"impl_72"}
// Dependencies: {}
impl Builder < Suffix , Icon > { # [doc = " Complete the [`Tag`]."] # [doc = ""] # [doc = " This can only be called once a suffix and an icon have been provided via"] # [doc = " [`.suffix(...)`](Builder::suffix) and [`.icon(...)`](Builder::icon), or"] # [doc = " alternatively just [`.level(...)`](Builder::level)."] pub fn build (self) -> Tag { Tag { prefix : self . prefix , suffix : self . suffix . 0 , icon : self . icon . 0 , } } }
};
}
