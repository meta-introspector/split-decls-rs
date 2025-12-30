// Generated macro for impl_988 (impl)
macro_rules! Depcrate_unknown_networkimpl_988 {
() => {
// Module: crate::unknown::network
// Provides: {"impl_988"}
// Dependencies: {}
impl NetworksInner { pub (crate) fn new () -> Self { Self { interfaces : HashMap :: new () , } } pub (crate) fn list (& self) -> & HashMap < String , NetworkData > { & self . interfaces } pub (crate) fn refresh (& mut self , _remove_not_listed_interfaces : bool) { } }
};
}
