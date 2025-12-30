// Generated macro for impl_227 (impl)
macro_rules! Depcrate_itemimpl_227 {
() => {
// Module: crate::item
// Provides: {"impl_227"}
// Dependencies: {}
impl AddRef { fn class (& self) -> Option < ItemClass > { match self { Self :: Key (_) => Some (ItemClass :: key ()) , Self :: Identity (_) => None , Self :: Certificate (_) => Some (ItemClass :: certificate ()) , } } fn ref_ (& self) -> CFTypeRef { match self { Self :: Key (key) => key . as_CFTypeRef () , Self :: Identity (id) => id . as_CFTypeRef () , Self :: Certificate (cert) => cert . as_CFTypeRef () , } } }
};
}
