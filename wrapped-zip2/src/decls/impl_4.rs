macro_rules! deps {
    () => {
        Aes128!();
        Aes256!();
        AesMode!();
        Aes192!();
        AesCtrZipKeyStream!();
        Cipher!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Cipher { # [doc = " Create a `Cipher` depending on the used `AesMode` and the given `key`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if `key` doesn't have the correct size for the chosen aes mode."] fn from_mode (aes_mode : AesMode , key : & [u8]) -> Self { match aes_mode { AesMode :: Aes128 => Cipher :: Aes128 (Box :: new (aes_ctr :: AesCtrZipKeyStream :: < aes_ctr :: Aes128 , > :: new (key))) , AesMode :: Aes192 => Cipher :: Aes192 (Box :: new (aes_ctr :: AesCtrZipKeyStream :: < aes_ctr :: Aes192 , > :: new (key))) , AesMode :: Aes256 => Cipher :: Aes256 (Box :: new (aes_ctr :: AesCtrZipKeyStream :: < aes_ctr :: Aes256 , > :: new (key))) , } } fn crypt_in_place (& mut self , target : & mut [u8]) { match self { Self :: Aes128 (cipher) => cipher . crypt_in_place (target) , Self :: Aes192 (cipher) => cipher . crypt_in_place (target) , Self :: Aes256 (cipher) => cipher . crypt_in_place (target) , } } }
    };
}

impl_4!();