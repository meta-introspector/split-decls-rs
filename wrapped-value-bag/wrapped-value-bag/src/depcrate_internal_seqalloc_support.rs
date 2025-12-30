// Generated macro for alloc_support (module)
macro_rules! Depcrate_internal_seqalloc_support {
() => {
// Module: crate::internal::seq
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: borrow :: Cow ; impl < 'v > ValueBag < 'v > { # [doc = " Try get a collection `S` of strings from this value."] # [doc = ""] # [doc = " If this value is a sequence then the collection `S` will be extended"] # [doc = " with the attempted conversion of each of its elements."] # [doc = ""] # [doc = " If this value is not a sequence then this method will return `None`."] # [inline] pub fn to_str_seq < S : Default + Extend < Option < Cow < 'v , str > > > > (& self) -> Option < S > { # [derive (Default)] struct ExtendStr < 'a , S > (S , PhantomData < Cow < 'a , str > >) ; impl < 'a , S : Extend < Option < Cow < 'a , str > > > > ExtendValue < 'a > for ExtendStr < 'a , S > { fn extend (& mut self , inner : Internal < '_ >) { self . 0 . extend (Some (ValueBag { inner } . to_str () . map (| s | Cow :: Owned (s . into_owned ())) ,)) } fn extend_borrowed (& mut self , inner : Internal < 'a >) { self . 0 . extend (Some (ValueBag { inner } . to_str ())) } } self . inner . extend :: < ExtendStr < 'v , S > > () . map (| seq | seq . 0) } } }
};
}
