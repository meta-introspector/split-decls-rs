macro_rules! AesCipher {
    () => {
        # [doc = " This trait allows using generic AES ciphers with different key sizes."] pub trait AesCipher { fn crypt_in_place (& mut self , target : & mut [u8]) ; }
    };
}

AesCipher!();