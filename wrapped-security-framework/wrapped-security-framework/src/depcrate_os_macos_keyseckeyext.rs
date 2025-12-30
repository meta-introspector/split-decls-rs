// Generated macro for SecKeyExt (trait)
macro_rules! Depcrate_os_macos_keySecKeyExt {
() => {
// Module: crate::os::macos::key
// Provides: {"SecKeyExt"}
// Dependencies: {}
# [doc = " An extension trait adding OSX specific functionality to `SecKey`."] pub trait SecKeyExt { # [doc = " Creates a new `SecKey` from a buffer containing key data."] fn from_data (key_type : KeyType , key_data : & CFData) -> Result < SecKey , CFError > ; }
};
}
