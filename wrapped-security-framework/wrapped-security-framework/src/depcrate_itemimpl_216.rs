// Generated macro for impl_216 (impl)
macro_rules! Depcrate_itemimpl_216 {
() => {
// Module: crate::item
// Provides: {"impl_216"}
// Dependencies: {}
# [cfg (target_os = "macos")] impl ItemSearchOptions { # [doc = " Search within the specified macOS keychains."] # [doc = ""] # [doc = " If this is not called, the default keychain will be searched."] # [inline] pub fn keychains (& mut self , keychains : & [SecKeychain]) -> & mut Self { self . keychains = Some (CFArray :: from_CFTypes (keychains)) ; self } # [doc = " Only search the protected data macOS keychains."] # [doc = ""] # [doc = " Has no effect if a legacy keychain has been explicitly specified"] # [doc = " using [keychains](ItemSearchOptions::keychains)."] # [doc = ""] # [doc = " Has no effect except in sandboxed applications on macOS 10.15 and above"] # [inline] pub fn ignore_legacy_keychains (& mut self) -> & mut Self { self . ignore_legacy_keychains = true ; self } }
};
}
