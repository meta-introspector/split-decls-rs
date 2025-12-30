// Generated macro for impl_249 (impl)
macro_rules! Depcrate_specimpl_249 {
() => {
// Module: crate::spec
// Provides: {"impl_249"}
// Dependencies: {}
impl FixedSizeBlock for Zip64CDELocatorBlock { const MAGIC : Magic = Magic :: ZIP64_CENTRAL_DIRECTORY_END_LOCATOR_SIGNATURE ; # [inline (always)] fn magic (self) -> Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid zip64 locator digital signature header") ; to_and_from_le ! [(magic , Magic) , (disk_with_central_directory , u32) , (end_of_central_directory_offset , u64) , (number_of_disks , u32) ,] ; }
};
}
