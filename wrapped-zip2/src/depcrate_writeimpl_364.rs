// Generated macro for impl_364 (impl)
macro_rules! Depcrate_writeimpl_364 {
() => {
// Module: crate::write
// Provides: {"impl_364"}
// Dependencies: {}
impl < W > Debug for MaybeEncrypted < W > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { MaybeEncrypted :: Unencrypted (_) => "Unencrypted" , # [cfg (feature = "aes-crypto")] MaybeEncrypted :: Aes (_) => "AES" , MaybeEncrypted :: ZipCrypto (_) => "ZipCrypto" , }) } }
};
}
