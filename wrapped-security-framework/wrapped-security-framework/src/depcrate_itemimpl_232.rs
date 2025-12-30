// Generated macro for impl_232 (impl)
macro_rules! Depcrate_itemimpl_232 {
() => {
// Module: crate::item
// Provides: {"impl_232"}
// Dependencies: {}
impl fmt :: Debug for Location { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { # [cfg (any (feature = "OSX_10_15" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] Self :: DataProtectionKeychain => "DataProtectionKeychain" , # [cfg (target_os = "macos")] Self :: DefaultFileKeychain => "DefaultFileKeychain" , # [cfg (target_os = "macos")] Self :: FileKeychain (_) => "FileKeychain" , }) } }
};
}
