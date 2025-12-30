// Generated macro for impl_113 (impl)
macro_rules! Depcrate_attrimpl_113 {
() => {
// Module: crate::attr
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (feature = "printing")] impl < 'a > FilterAttrs < 'a > for & 'a [Attribute] { type Ret = iter :: Filter < slice :: Iter < 'a , Attribute > , fn (& & Attribute) -> bool > ; fn outer (self) -> Self :: Ret { fn is_outer (attr : & & Attribute) -> bool { match attr . style { AttrStyle :: Outer => true , AttrStyle :: Inner (_) => false , } } self . iter () . filter (is_outer) } # [cfg (feature = "full")] fn inner (self) -> Self :: Ret { fn is_inner (attr : & & Attribute) -> bool { match attr . style { AttrStyle :: Inner (_) => true , AttrStyle :: Outer => false , } } self . iter () . filter (is_inner) } }
};
}
