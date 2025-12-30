// Generated macro for impl_110 (impl)
macro_rules! Depcrate_nameimpl_110 {
() => {
// Module: crate::name
// Provides: {"impl_110"}
// Dependencies: {}
impl RelativeDistinguishedName { # [doc = " Is this [`RelativeDistinguishedName`] empty?"] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Iterate over this [`RelativeDistinguishedName`]."] pub fn iter (& self) -> impl Iterator < Item = & AttributeTypeAndValue > { self . 0 . iter () } # [doc = " Length of this [`RelativeDistinguishedName`]."] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Insert an [`AttributeTypeAndValue`] into this [`RelativeDistinguishedName`]. Must be unique."] pub fn insert (& mut self , item : AttributeTypeAndValue) -> Result < () , der :: Error > { self . 0 . insert (item) } }
};
}
