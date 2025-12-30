// Generated macro for impl_256 (impl)
macro_rules! Depcrate_deimpl_256 {
() => {
// Module: crate::de
// Provides: {"impl_256"}
// Dependencies: {}
impl BorrowedLifetimes { fn de_lifetime (& self) -> syn :: Lifetime { match * self { BorrowedLifetimes :: Borrowed (_) => syn :: Lifetime :: new ("'de" , Span :: call_site ()) , BorrowedLifetimes :: Static => syn :: Lifetime :: new ("'static" , Span :: call_site ()) , } } fn de_lifetime_param (& self) -> Option < syn :: LifetimeParam > { match self { BorrowedLifetimes :: Borrowed (bounds) => Some (syn :: LifetimeParam { attrs : Vec :: new () , lifetime : syn :: Lifetime :: new ("'de" , Span :: call_site ()) , colon_token : None , bounds : bounds . iter () . cloned () . collect () , }) , BorrowedLifetimes :: Static => None , } } }
};
}
