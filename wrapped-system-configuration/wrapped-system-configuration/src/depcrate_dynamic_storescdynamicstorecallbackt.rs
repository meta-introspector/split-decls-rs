// Generated macro for SCDynamicStoreCallBackT (type)
macro_rules! Depcrate_dynamic_storeSCDynamicStoreCallBackT {
() => {
// Module: crate::dynamic_store
// Provides: {"SCDynamicStoreCallBackT"}
// Dependencies: {}
# [doc = " Signature for callback functions getting called when a watched value in the dynamic store is"] # [doc = " changed."] # [doc = ""] # [doc = " This is the safe callback definition, abstracting over the lower level `SCDynamicStoreCallBack`"] # [doc = " from the `system-configuration-sys` crate."] pub type SCDynamicStoreCallBackT < T > = fn (store : SCDynamicStore , changed_keys : CFArray < CFString > , info : & mut T) ;
};
}
