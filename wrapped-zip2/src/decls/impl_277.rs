macro_rules! deps {
    () => {
        ZipCryptoReaderValid!();
        ZipError!();
        ZipCryptoKeys!();
        ZipCryptoValidator!();
        ZipCryptoReader!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < R : std :: io :: Read > ZipCryptoReader < R > { # [doc = " Note: The password is `&[u8]` and not `&str` because the"] # [doc = " [zip specification](https://pkware.cachefly.net/webdocs/APPNOTE/APPNOTE-6.3.3.TXT)"] # [doc = " does not specify password encoding (see function `update_keys` in the specification)."] # [doc = " Therefore, if `&str` was used, the password would be UTF-8 and it"] # [doc = " would be impossible to decrypt files that were encrypted with a"] # [doc = " password byte sequence that is unrepresentable in UTF-8."] pub fn new (file : R , password : & [u8]) -> ZipCryptoReader < R > { ZipCryptoReader { file , keys : ZipCryptoKeys :: derive (password) , } } # [doc = " Read the ZipCrypto header bytes and validate the password."] pub fn validate (mut self , validator : ZipCryptoValidator ,) -> Result < ZipCryptoReaderValid < R > , ZipError > { let mut header_buf = [0u8 ; 12] ; self . file . read_exact (& mut header_buf) ? ; for byte in header_buf . iter_mut () { * byte = self . keys . decrypt_byte (* byte) ; } match validator { ZipCryptoValidator :: PkzipCrc32 (crc32_plaintext) => { if (crc32_plaintext >> 24) as u8 != header_buf [11] { return Err (ZipError :: InvalidPassword) ; } } ZipCryptoValidator :: InfoZipMsdosTime (last_mod_time) => { if (last_mod_time >> 8) as u8 != header_buf [11] { return Err (ZipError :: InvalidPassword) ; } } } Ok (ZipCryptoReaderValid { reader : self }) } }
    };
}

impl_277!()