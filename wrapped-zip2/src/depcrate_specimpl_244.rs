// Generated macro for impl_244 (impl)
macro_rules! Depcrate_specimpl_244 {
() => {
// Module: crate::spec
// Provides: {"impl_244"}
// Dependencies: {}
impl FixedSizeBlock for Zip32CDEBlock { const MAGIC : Magic = Magic :: CENTRAL_DIRECTORY_END_SIGNATURE ; # [inline (always)] fn magic (self) -> Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid digital signature header") ; to_and_from_le ! [(magic , Magic) , (disk_number , u16) , (disk_with_central_directory , u16) , (number_of_files_on_this_disk , u16) , (number_of_files , u16) , (central_directory_size , u32) , (central_directory_offset , u32) , (zip_file_comment_length , u16)] ; }
};
}
