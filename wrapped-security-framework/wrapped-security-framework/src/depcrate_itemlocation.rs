// Generated macro for Location (enum)
macro_rules! Depcrate_itemLocation {
() => {
// Module: crate::item
// Provides: {"Location"}
// Dependencies: {}
# [doc = " Which keychain to add an item to."] # [doc = ""] # [doc = " <https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains>"] pub enum Location { # [doc = " Store the item in the newer `DataProtectionKeychain`. This is the only"] # [doc = " keychain on iOS. On macOS, this is the newer and more consistent"] # [doc = " keychain implementation. Keys stored in the Secure Enclave _must_ use"] # [doc = " this keychain."] # [doc = ""] # [doc = " This keychain requires the calling binary to be codesigned with"] # [doc = " entitlements for the `KeychainAccessGroups` it is supposed to"] # [doc = " access."] # [cfg (any (feature = "OSX_10_15" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] DataProtectionKeychain , # [doc = " Store the key in the default file-based keychain. On macOS, defaults to"] # [doc = " the Login keychain."] # [cfg (target_os = "macos")] DefaultFileKeychain , # [doc = " Store the key in a specific file-based keychain."] # [cfg (target_os = "macos")] FileKeychain (crate :: os :: macos :: keychain :: SecKeychain) , }
};
}
