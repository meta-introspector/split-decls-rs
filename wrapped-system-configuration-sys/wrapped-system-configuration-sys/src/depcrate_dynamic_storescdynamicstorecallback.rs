// Generated macro for SCDynamicStoreCallBack (type)
macro_rules! Depcrate_dynamic_storeSCDynamicStoreCallBack {
() => {
// Module: crate::dynamic_store
// Provides: {"SCDynamicStoreCallBack"}
// Dependencies: {}
pub type SCDynamicStoreCallBack = Option < unsafe extern "C" fn (store : SCDynamicStoreRef , changedKeys : CFArrayRef , info : * mut :: core :: ffi :: c_void ,) , > ;
};
}
