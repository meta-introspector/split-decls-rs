// Generated macro for SCDynamicStoreBuilder (struct)
macro_rules! Depcrate_dynamic_storeSCDynamicStoreBuilder {
() => {
// Module: crate::dynamic_store
// Provides: {"SCDynamicStoreBuilder"}
// Dependencies: {}
# [doc = " Builder for [`SCDynamicStore`] sessions."] # [doc = ""] # [doc = " [`SCDynamicStore`]: struct.SCDynamicStore.html"] pub struct SCDynamicStoreBuilder < T > { name : CFString , session_keys : bool , callback_context : Option < SCDynamicStoreCallBackContext < T > > , }
};
}
