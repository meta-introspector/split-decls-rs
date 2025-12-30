// Generated macro for make_reader (function)
macro_rules! Depcrate_readmake_reader {
() => {
// Module: crate::read
// Provides: {"make_reader"}
// Dependencies: {}
pub (crate) fn make_reader < R : Read > (compression_method : CompressionMethod , uncompressed_size : u64 , crc32 : u32 , reader : CryptoReader < R > , flags : u16 ,) -> ZipResult < ZipFileReader < R > > { let ae2_encrypted = reader . is_ae2_encrypted () ; Ok (ZipFileReader :: Compressed (Box :: new (Crc32Reader :: new (Decompressor :: new (io :: BufReader :: new (reader) , compression_method , uncompressed_size , flags ,) ? , crc32 , ae2_encrypted ,)))) }
};
}
