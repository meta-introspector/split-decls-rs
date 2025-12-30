// Generated macro for make_crypto_reader (function)
macro_rules! Depcrate_readmake_crypto_reader {
() => {
// Module: crate::read
// Provides: {"make_crypto_reader"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub (crate) fn make_crypto_reader < 'a , R : Read > (data : & ZipFileData , reader : io :: Take < & 'a mut R > , password : Option < & [u8] > , aes_info : Option < (AesMode , AesVendorVersion , CompressionMethod) > ,) -> ZipResult < CryptoReader < 'a , R > > { # [allow (deprecated)] { if let CompressionMethod :: Unsupported (_) = data . compression_method { return unsupported_zip_error ("Compression method not supported") ; } } let reader = match (password , aes_info) { # [cfg (not (feature = "aes-crypto"))] (Some (_) , Some (_)) => { return Err (ZipError :: UnsupportedArchive ("AES encrypted files cannot be decrypted without the aes-crypto feature." ,)) } # [cfg (feature = "aes-crypto")] (Some (password) , Some ((aes_mode , vendor_version , _))) => CryptoReader :: Aes { reader : AesReader :: new (reader , aes_mode , data . compressed_size) . validate (password) ? , vendor_version , } , (Some (password) , None) => { let validator = if data . using_data_descriptor { ZipCryptoValidator :: InfoZipMsdosTime (data . last_modified_time . map_or (0 , | x | x . timepart ()) ,) } else { ZipCryptoValidator :: PkzipCrc32 (data . crc32) } ; CryptoReader :: ZipCrypto (ZipCryptoReader :: new (reader , password) . validate (validator) ?) } (None , Some (_)) => return Err (InvalidPassword) , (None , None) => CryptoReader :: Plaintext (reader) , } ; Ok (reader) }
};
}
