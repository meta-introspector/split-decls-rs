// Generated macro for find_data_start (function)
macro_rules! Depcrate_readfind_data_start {
() => {
// Module: crate::read
// Provides: {"find_data_start"}
// Dependencies: {}
pub (crate) fn find_data_start (data : & ZipFileData , reader : & mut (impl Read + Seek + Sized) ,) -> Result < u64 , ZipError > { reader . seek (SeekFrom :: Start (data . header_start)) ? ; let block = ZipLocalEntryBlock :: parse (reader) ? ; let variable_fields_len = block . file_name_length as u64 + block . extra_field_length as u64 ; let data_start = data . header_start + size_of :: < ZipLocalEntryBlock > () as u64 + variable_fields_len ; match data . data_start . set (data_start) { Ok (()) => () , Err (_) => { debug_assert_eq ! (* data . data_start . get () . unwrap () , data_start) ; } } Ok (data_start) }
};
}
