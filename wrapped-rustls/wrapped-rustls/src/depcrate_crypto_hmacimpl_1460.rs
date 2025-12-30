// Generated macro for impl_1460 (impl)
macro_rules! Depcrate_crypto_hmacimpl_1460 {
() => {
// Module: crate::crypto::hmac
// Provides: {"impl_1460"}
// Dependencies: {}
impl Tag { # [doc = " Build a tag by copying a byte slice."] # [doc = ""] # [doc = " The slice can be up to [`Tag::MAX_LEN`] bytes in length."] pub fn new (bytes : & [u8]) -> Self { Self (PublicTag :: new (bytes)) } # [doc = " Declare this tag is public."] # [doc = ""] # [doc = " Uses of this function should explain why this tag is public."] pub (crate) fn into_public (self) -> PublicTag { let public = self . 0 . clone () ; mem :: forget (self) ; public } # [doc = " Maximum supported HMAC tag size: supports up to SHA512."] pub const MAX_LEN : usize = 64 ; }
};
}
