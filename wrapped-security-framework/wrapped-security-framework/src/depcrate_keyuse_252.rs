// Generated macro for use_252 (use)
macro_rules! Depcrate_keyuse_252 {
() => {
// Module: crate::key
// Provides: {"use_252"}
// Dependencies: {}
# [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] use security_framework_sys :: key :: { SecKeyCopyAttributes , SecKeyCopyExternalRepresentation , SecKeyCreateSignature , SecKeyCreateRandomKey , SecKeyCopyPublicKey , SecKeyCreateDecryptedData , SecKeyCreateEncryptedData , } ;
};
}
