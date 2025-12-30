// Generated macro for impl_97 (impl)
macro_rules! Depcrate_nameimpl_97 {
() => {
// Module: crate::name
// Provides: {"impl_97"}
// Dependencies: {}
impl Name { # [doc = " Is this [`Name`] empty?"] # [inline] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Returns the number of [`RelativeDistinguishedName`] elements in this [`Name`]."] pub fn len (& self) -> usize { self . 0 . 0 . len () } # [doc = " Returns an iterator over the inner [`AttributeTypeAndValue`]s."] # [doc = ""] # [doc = " This iterator does not expose which attributes are grouped together as"] # [doc = " [`RelativeDistinguishedName`]s. If you need this, use [`Self::iter_rdn`]."] # [inline] pub fn iter (& self) -> impl Iterator < Item = & '_ AttributeTypeAndValue > + '_ { self . 0 . 0 . iter () . flat_map (move | rdn | rdn . 0 . as_slice ()) } # [doc = " Returns an iterator over the inner [`RelativeDistinguishedName`]s."] # [inline] pub fn iter_rdn (& self) -> impl Iterator < Item = & '_ RelativeDistinguishedName > + '_ { self . 0 . 0 . iter () } }
};
}
