macro_rules! deps {
    () => {
        ZipCryptoKeys!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl ZipCryptoKeys { const fn new () -> ZipCryptoKeys { ZipCryptoKeys { key_0 : Wrapping (0x12345678) , key_1 : Wrapping (0x23456789) , key_2 : Wrapping (0x34567890) , } } # [allow (unused)] pub const fn of (key_0 : u32 , key_1 : u32 , key_2 : u32) -> ZipCryptoKeys { ZipCryptoKeys { key_0 : Wrapping (key_0) , key_1 : Wrapping (key_1) , key_2 : Wrapping (key_2) , } } fn update (& mut self , input : u8) { self . key_0 = ZipCryptoKeys :: crc32 (self . key_0 , input) ; self . key_1 = (self . key_1 + (self . key_0 & Wrapping (0xff))) * Wrapping (0x08088405) + Wrapping (1) ; self . key_2 = ZipCryptoKeys :: crc32 (self . key_2 , (self . key_1 >> 24) . 0 as u8) ; } fn stream_byte (& mut self) -> u8 { let temp : Wrapping < u16 > = Wrapping (self . key_2 . 0 as u16) | Wrapping (3) ; ((temp * (temp ^ Wrapping (1))) >> 8) . 0 as u8 } fn decrypt_byte (& mut self , cipher_byte : u8) -> u8 { let plain_byte : u8 = self . stream_byte () ^ cipher_byte ; self . update (plain_byte) ; plain_byte } # [allow (dead_code)] fn encrypt_byte (& mut self , plain_byte : u8) -> u8 { let cipher_byte : u8 = self . stream_byte () ^ plain_byte ; self . update (plain_byte) ; cipher_byte } fn crc32 (crc : Wrapping < u32 > , input : u8) -> Wrapping < u32 > { (crc >> 8) ^ Wrapping (CRCTABLE [((crc & Wrapping (0xff)) . 0 as u8 ^ input) as usize]) } pub (crate) fn derive (password : & [u8]) -> ZipCryptoKeys { let mut keys = ZipCryptoKeys :: new () ; for byte in password . iter () { keys . update (* byte) ; } keys } }
    };
}

impl_274!()