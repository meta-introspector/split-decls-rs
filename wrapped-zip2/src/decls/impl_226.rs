macro_rules! deps {
    () => {
        Aes192!();
        AesMode!();
        Aes128!();
        Aes256!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        # [cfg (feature = "aes-crypto")] impl AesMode { # [doc = " Length of the salt for the given AES mode."] pub const fn salt_length (& self) -> usize { self . key_length () / 2 } # [doc = " Length of the key for the given AES mode."] pub const fn key_length (& self) -> usize { match self { Self :: Aes128 => 16 , Self :: Aes192 => 24 , Self :: Aes256 => 32 , } } }
    };
}

impl_226!()