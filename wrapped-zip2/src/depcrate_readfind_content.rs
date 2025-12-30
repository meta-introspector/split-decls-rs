// Generated macro for find_content (function)
macro_rules! Depcrate_readfind_content {
() => {
// Module: crate::read
// Provides: {"find_content"}
// Dependencies: {}
pub (crate) fn find_content < 'a , R : Read + Seek > (data : & ZipFileData , reader : & 'a mut R ,) -> ZipResult < io :: Take < & 'a mut R > > { let data_start = data . data_start (reader) ? ; reader . seek (SeekFrom :: Start (data_start)) ? ; Ok (reader . take (data . compressed_size)) }
};
}
