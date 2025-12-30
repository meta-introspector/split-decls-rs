// Generated macro for impl_28 (impl)
macro_rules! Depcrate_attrimpl_28 {
() => {
// Module: crate::attr
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a , T > FilterAttrs < 'a > for T where T : IntoIterator < Item = & 'a Attribute > { type Ret = iter :: Filter < T :: IntoIter , fn (& & Attribute) -> bool > ; fn outer (self) -> Self :: Ret { fn is_outer (attr : & & Attribute) -> bool { match attr . style { AttrStyle :: Outer => true , _ => false , } } self . into_iter () . filter (is_outer) } fn inner (self) -> Self :: Ret { fn is_inner (attr : & & Attribute) -> bool { match attr . style { AttrStyle :: Inner (_) => true , _ => false , } } self . into_iter () . filter (is_inner) } }
};
}
