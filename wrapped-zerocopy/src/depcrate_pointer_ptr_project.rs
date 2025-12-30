// Generated macro for _project (module)
macro_rules! Depcrate_pointer_ptr_project {
() => {
// Module: crate::pointer::ptr
// Provides: {"_project"}
// Dependencies: {}
# [doc = " Projections through the referent."] mod _project { use super :: * ; impl < 'a , T , I > Ptr < 'a , [T] , I > where T : 'a , I : Invariants , I :: Aliasing : Reference , { # [doc = " Iteratively projects the elements `Ptr<T>` from `Ptr<[T]>`."] pub (crate) fn iter (& self) -> impl Iterator < Item = Ptr < 'a , T , I > > { self . as_inner () . iter () . map (| elem | unsafe { Ptr :: from_inner (elem) }) } } # [allow (clippy :: needless_lifetimes)] impl < 'a , T , I > Ptr < 'a , T , I > where T : 'a + ? Sized + KnownLayout < PointerMetadata = usize > , I : Invariants , { # [doc = " The number of slice elements in the object referenced by `self`."] pub (crate) fn len (& self) -> usize { self . as_inner () . meta () . get () } } }
};
}
