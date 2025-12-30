// Generated macro for find_content_seek (function)
macro_rules! Depcrate_readfind_content_seek {
() => {
// Module: crate::read
// Provides: {"find_content_seek"}
// Dependencies: {}
fn find_content_seek < 'a , R : Read + Seek > (data : & ZipFileData , reader : & 'a mut R ,) -> ZipResult < SeekableTake < 'a , R > > { let data_start = data . data_start (reader) ? ; reader . seek (SeekFrom :: Start (data_start)) ? ; Ok (SeekableTake :: new (reader , data . compressed_size) ?) }
};
}
