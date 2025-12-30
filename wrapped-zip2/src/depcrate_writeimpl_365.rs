// Generated macro for impl_365 (impl)
macro_rules! Depcrate_writeimpl_365 {
() => {
// Module: crate::write
// Provides: {"impl_365"}
// Dependencies: {}
impl < W : Write > Write for MaybeEncrypted < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match self { MaybeEncrypted :: Unencrypted (w) => w . write (buf) , # [cfg (feature = "aes-crypto")] MaybeEncrypted :: Aes (w) => w . write (buf) , MaybeEncrypted :: ZipCrypto (w) => w . write (buf) , } } fn flush (& mut self) -> io :: Result < () > { match self { MaybeEncrypted :: Unencrypted (w) => w . flush () , # [cfg (feature = "aes-crypto")] MaybeEncrypted :: Aes (w) => w . flush () , MaybeEncrypted :: ZipCrypto (w) => w . flush () , } } }
};
}
