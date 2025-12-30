// Generated macro for ZipFileReader (enum)
macro_rules! Depcrate_readZipFileReader {
() => {
// Module: crate::read
// Provides: {"ZipFileReader"}
// Dependencies: {}
pub (crate) enum ZipFileReader < 'a , R : Read > { NoReader , Raw (io :: Take < & 'a mut R >) , Compressed (Box < Crc32Reader < Decompressor < io :: BufReader < CryptoReader < 'a , R > > > > >) , }
};
}
