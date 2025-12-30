// Generated macro for convert_callback (function)
macro_rules! Depcrate_dynamic_storeconvert_callback {
() => {
// Module: crate::dynamic_store
// Provides: {"convert_callback"}
// Dependencies: {}
# [doc = " The raw callback used by the safe `SCDynamicStore` to convert from the `SCDynamicStoreCallBack`"] # [doc = " to the `SCDynamicStoreCallBackT`"] unsafe extern "C" fn convert_callback < T > (store_ref : SCDynamicStoreRef , changed_keys_ref : CFArrayRef , context_ptr : * mut c_void ,) { let store = SCDynamicStore :: wrap_under_get_rule (store_ref) ; let changed_keys = CFArray :: < CFString > :: wrap_under_get_rule (changed_keys_ref) ; let context = & mut * (context_ptr as * mut _ as * mut SCDynamicStoreCallBackContext < T >) ; (context . callout) (store , changed_keys , & mut context . info) ; }
};
}
