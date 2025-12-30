// Generated macro for impl_10 (impl)
macro_rules! Depcrate_dynamic_storeimpl_10 {
() => {
// Module: crate::dynamic_store
// Provides: {"impl_10"}
// Dependencies: {}
impl SCDynamicStoreBuilder < () > { # [doc = " Creates a new builder. `name` is used as the name parameter when creating the"] # [doc = " [`SCDynamicStore`] session."] # [doc = ""] # [doc = " [`SCDynamicStore`]: struct.SCDynamicStore.html"] pub fn new < S : Into < CFString > > (name : S) -> Self { SCDynamicStoreBuilder { name : name . into () , session_keys : false , callback_context : None , } } }
};
}
