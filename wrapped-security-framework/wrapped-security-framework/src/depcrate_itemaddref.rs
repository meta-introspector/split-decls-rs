// Generated macro for AddRef (enum)
macro_rules! Depcrate_itemAddRef {
() => {
// Module: crate::item
// Provides: {"AddRef"}
// Dependencies: {}
# [doc = " Type of Ref to add to the keychain."] pub enum AddRef { # [doc = " `SecKey`"] Key (SecKey) , # [doc = " `SecIdentity`"] Identity (SecIdentity) , # [doc = " `SecCertificate`"] Certificate (SecCertificate) , }
};
}
