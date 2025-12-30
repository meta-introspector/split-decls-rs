// Generated macro for SCDynamicStoreCallBackContext (struct)
macro_rules! Depcrate_dynamic_storeSCDynamicStoreCallBackContext {
() => {
// Module: crate::dynamic_store
// Provides: {"SCDynamicStoreCallBackContext"}
// Dependencies: {}
# [doc = " Struct describing the callback happening when a watched value in the dynamic store is changed."] pub struct SCDynamicStoreCallBackContext < T > { # [doc = " The callback function that will be called when a watched value in the dynamic store is"] # [doc = " changed."] pub callout : SCDynamicStoreCallBackT < T > , # [doc = " The argument passed to each `callout` call. Can be used to keep state between"] # [doc = " callbacks."] pub info : T , }
};
}
