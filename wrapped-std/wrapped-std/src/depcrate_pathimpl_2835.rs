// Generated macro for impl_2835 (impl)
macro_rules! Depcrate_pathimpl_2835 {
() => {
// Module: crate::path
// Provides: {"impl_2835"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > PartialEq for Components < 'a > { # [inline] fn eq (& self , other : & Components < 'a >) -> bool { let Components { path : _ , front : _ , back : _ , has_physical_root : _ , prefix : _ } = self ; if self . path . len () == other . path . len () && self . front == other . front && self . back == State :: Body && other . back == State :: Body && self . prefix_verbatim () == other . prefix_verbatim () { if self . path == other . path { return true ; } } Iterator :: eq (self . clone () . rev () , other . clone () . rev ()) } }
};
}
