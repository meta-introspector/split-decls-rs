// Generated macro for ItemSearchOptionsExt (trait)
macro_rules! Depcrate_os_macos_itemItemSearchOptionsExt {
() => {
// Module: crate::os::macos::item
// Provides: {"ItemSearchOptionsExt"}
// Dependencies: {}
# [doc (hidden)] # [doc = " An obsolete trait for `ItemSearchOptions`. Use methods on `ItemSearchOptions` directly."] pub trait ItemSearchOptionsExt { # [doc = " Search within the specified keychains."] # [doc = ""] # [doc = " If this is not called, the default keychain will be searched."] fn keychains (& mut self , keychains : & [SecKeychain]) -> & mut Self ; }
};
}
