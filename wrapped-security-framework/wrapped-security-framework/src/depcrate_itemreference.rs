// Generated macro for Reference (enum)
macro_rules! Depcrate_itemReference {
() => {
// Module: crate::item
// Provides: {"Reference"}
// Dependencies: {}
# [doc = " An enum including all objects whose references can be returned from a search."] # [doc = ""] # [doc = " Note that generic _Keychain Items_, such as passwords and preferences, do"] # [doc = " not have specific object types; they are modeled using dictionaries and so"] # [doc = " are available directly as search results in variant `SearchResult::Dict`."] # [derive (Debug)] pub enum Reference { # [doc = " A `SecIdentity`."] Identity (SecIdentity) , # [doc = " A `SecCertificate`."] Certificate (SecCertificate) , # [doc = " A `SecKey`."] Key (SecKey) , # [doc = " A `SecKeychainItem`."] # [doc = ""] # [doc = " Only defined on OSX"] # [cfg (target_os = "macos")] KeychainItem (crate :: os :: macos :: keychain_item :: SecKeychainItem) , # [doc (hidden)] __NonExhaustive , }
};
}
