// Generated macro for GenerateKeyOptions (struct)
macro_rules! Depcrate_keyGenerateKeyOptions {
() => {
// Module: crate::key
// Provides: {"GenerateKeyOptions"}
// Dependencies: {}
# [doc = " Helper for creating `CFDictionary` attributes for `SecKey::generate`"] # [doc = " Recommended reading:"] # [doc = " <https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains>"] # [derive (Debug , Default)] # [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] pub struct GenerateKeyOptions { # [doc = " kSecAttrKeyType"] # [deprecated (note = "use set_key_type()")] pub key_type : Option < KeyType > , # [doc = " kSecAttrKeySizeInBits"] # [deprecated (note = "use set_size_in_bits()")] pub size_in_bits : Option < u32 > , # [doc = " kSecAttrLabel"] # [deprecated (note = "use set_label()")] pub label : Option < String > , # [doc = " kSecAttrTokenID"] # [deprecated (note = "use set_token()")] pub token : Option < Token > , # [doc = " Which keychain to store the key in, if any."] # [deprecated (note = "use set_location()")] pub location : Option < Location > , # [doc = " Access control"] # [deprecated (note = "use set_access_control()")] pub access_control : Option < SecAccessControl > , # [doc = " `kSecAttrSynchronizable`"] # [cfg (feature = "sync-keychain")] synchronizable : Option < bool > , }
};
}
