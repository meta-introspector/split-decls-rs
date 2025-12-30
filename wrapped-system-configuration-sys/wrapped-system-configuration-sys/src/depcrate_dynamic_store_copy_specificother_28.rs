// Generated macro for other_28 (other)
macro_rules! Depcrate_dynamic_store_copy_specificother_28 {
() => {
// Module: crate::dynamic_store_copy_specific
// Provides: {"other_28"}
// Dependencies: {}
extern "C" { pub fn SCDynamicStoreCopyComputerName (store : SCDynamicStoreRef , nameEncoding : * mut CFStringEncoding ,) -> CFStringRef ; pub fn SCDynamicStoreCopyConsoleUser (store : SCDynamicStoreRef , uid : * mut uid_t , gid : * mut gid_t ,) -> CFStringRef ; pub fn SCDynamicStoreCopyLocalHostName (store : SCDynamicStoreRef) -> CFStringRef ; pub fn SCDynamicStoreCopyLocation (store : SCDynamicStoreRef) -> CFStringRef ; pub fn SCDynamicStoreCopyProxies (store : SCDynamicStoreRef) -> CFDictionaryRef ; }
};
}
