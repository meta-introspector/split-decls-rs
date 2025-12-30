// Generated macro for SliceLike (trait)
macro_rules! Depcrate_inherentSliceLike {
() => {
// Module: crate::inherent
// Provides: {"SliceLike"}
// Dependencies: {}
pub trait SliceLike : Sized + Copy { type Item : Copy ; type IntoIter : Iterator < Item = Self :: Item > + DoubleEndedIterator ; fn iter (self) -> Self :: IntoIter ; fn as_slice (& self) -> & [Self :: Item] ; fn get (self , idx : usize) -> Option < Self :: Item > { self . as_slice () . get (idx) . copied () } fn len (self) -> usize { self . as_slice () . len () } fn is_empty (self) -> bool { self . len () == 0 } fn contains (self , t : & Self :: Item) -> bool where Self :: Item : PartialEq , { self . as_slice () . contains (t) } fn to_vec (self) -> Vec < Self :: Item > { self . as_slice () . to_vec () } fn last (self) -> Option < Self :: Item > { self . as_slice () . last () . copied () } fn split_last (& self) -> Option < (& Self :: Item , & [Self :: Item]) > { self . as_slice () . split_last () } }
};
}
