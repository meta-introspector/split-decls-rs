// Generated macro for impl_172 (impl)
macro_rules! Depcrate_readimpl_172 {
() => {
// Module: crate::read
// Provides: {"impl_172"}
// Dependencies: {}
impl < R > ZipArchive < R > { pub (crate) fn from_finalized_writer (files : IndexMap < Box < str > , ZipFileData > , comment : Box < [u8] > , zip64_comment : Option < Box < [u8] > > , reader : R , central_start : u64 ,) -> ZipResult < Self > { let initial_offset = match files . first () { Some ((_ , file)) => file . header_start , None => central_start , } ; let shared = Arc :: new (Shared { files , offset : initial_offset , dir_start : central_start , config : Config { archive_offset : ArchiveOffset :: Known (initial_offset) , } , comment , zip64_comment , }) ; Ok (Self { reader , shared }) } # [doc = " Total size of the files in the archive, if it can be known. Doesn't include directories or"] # [doc = " metadata."] pub fn decompressed_size (& self) -> Option < u128 > { let mut total = 0u128 ; for file in self . shared . files . values () { if file . using_data_descriptor { return None ; } total = total . checked_add (file . uncompressed_size as u128) ? ; } Some (total) } }
};
}
