// Generated macro for other_402 (other)
macro_rules! Depcrate_keychain_itemother_402 {
() => {
// Module: crate::keychain_item
// Provides: {"other_402"}
// Dependencies: {}
extern "C" { # [doc = " Returns the unique identifier of the opaque type to which a keychain item object belongs."] pub fn SecKeychainItemGetTypeID () -> CFTypeID ; # [doc = " Adds one or more items to a keychain."] pub fn SecItemAdd (attributes : CFDictionaryRef , result : * mut CFTypeRef) -> OSStatus ; # [doc = " Returns one or more keychain items that match a search query, or copies attributes of specific keychain items."] pub fn SecItemCopyMatching (query : CFDictionaryRef , result : * mut CFTypeRef) -> OSStatus ; # [doc = " Modifies items that match a search query."] pub fn SecItemUpdate (query : CFDictionaryRef , attributesToUpdate : CFDictionaryRef) -> OSStatus ; # [doc = " Deletes items that match a search query."] pub fn SecItemDelete (query : CFDictionaryRef) -> OSStatus ; # [doc = " # Legacy API"] pub fn SecKeychainItemModifyAttributesAndData (itemRef : SecKeychainItemRef , attrList : * const SecKeychainAttributeList , length : u32 , data : * const c_void ,) -> OSStatus ; pub fn SecKeychainItemFreeContent (attrList : * mut SecKeychainAttributeList , data : * mut c_void ,) -> OSStatus ; pub fn SecKeychainItemDelete (itemRef : SecKeychainItemRef) -> OSStatus ; }
};
}
