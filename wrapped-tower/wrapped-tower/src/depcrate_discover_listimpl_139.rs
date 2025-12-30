// Generated macro for impl_139 (impl)
macro_rules! Depcrate_discover_listimpl_139 {
() => {
// Module: crate::discover::list
// Provides: {"impl_139"}
// Dependencies: {}
impl < T , U > ServiceList < T > where T : IntoIterator < Item = U > , { # [allow (missing_docs)] pub fn new < Request > (services : T) -> ServiceList < T > where U : Service < Request > , { ServiceList { inner : services . into_iter () . enumerate () , } } }
};
}
