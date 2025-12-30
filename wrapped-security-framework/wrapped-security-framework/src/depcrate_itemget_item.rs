// Generated macro for get_item (function)
macro_rules! Depcrate_itemget_item {
() => {
// Module: crate::item
// Provides: {"get_item"}
// Dependencies: {}
unsafe fn get_item (item : CFTypeRef) -> SearchResult { let type_id = CFGetTypeID (item) ; if type_id == CFData :: type_id () { let data = CFData :: wrap_under_get_rule (item as * mut _) ; let mut buf = Vec :: new () ; buf . extend_from_slice (data . bytes ()) ; return SearchResult :: Data (buf) ; } if type_id == CFDictionary :: < * const u8 , * const u8 > :: type_id () { return SearchResult :: Dict (CFDictionary :: wrap_under_get_rule (item as * mut _)) ; } # [cfg (target_os = "macos")] { use crate :: os :: macos :: keychain_item :: SecKeychainItem ; if type_id == SecKeychainItem :: type_id () { return SearchResult :: Ref (Reference :: KeychainItem (SecKeychainItem :: wrap_under_get_rule (item as * mut _) ,)) ; } } let reference = if type_id == SecCertificate :: type_id () { Reference :: Certificate (SecCertificate :: wrap_under_get_rule (item as * mut _)) } else if type_id == SecKey :: type_id () { Reference :: Key (SecKey :: wrap_under_get_rule (item as * mut _)) } else if type_id == SecIdentity :: type_id () { Reference :: Identity (SecIdentity :: wrap_under_get_rule (item as * mut _)) } else { panic ! ("Got bad type from SecItemCopyMatching: {type_id}") ; } ; SearchResult :: Ref (reference) }
};
}
