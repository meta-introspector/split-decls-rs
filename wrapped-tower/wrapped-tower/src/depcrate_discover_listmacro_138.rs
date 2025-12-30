// Generated macro for macro_138 (macro)
macro_rules! Depcrate_discover_listmacro_138 {
() => {
// Module: crate::discover::list
// Provides: {"macro_138"}
// Dependencies: {}
pin_project ! { # [doc = " Static service discovery based on a predetermined list of services."] # [doc = ""] # [doc = " [`ServiceList`] is created with an initial list of services. The discovery"] # [doc = " process will yield this list once and do nothing after."] # [derive (Debug)] pub struct ServiceList < T > where T : IntoIterator , { inner : Enumerate < T :: IntoIter >, } }
};
}
