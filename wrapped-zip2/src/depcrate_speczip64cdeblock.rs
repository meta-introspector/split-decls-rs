// Generated macro for Zip64CDEBlock (struct)
macro_rules! Depcrate_specZip64CDEBlock {
() => {
// Module: crate::spec
// Provides: {"Zip64CDEBlock"}
// Dependencies: {}
# [derive (Copy , Clone)] # [repr (packed , C)] pub (crate) struct Zip64CDEBlock { magic : Magic , pub record_size : u64 , pub version_made_by : u16 , pub version_needed_to_extract : u16 , pub disk_number : u32 , pub disk_with_central_directory : u32 , pub number_of_files_on_this_disk : u64 , pub number_of_files : u64 , pub central_directory_size : u64 , pub central_directory_offset : u64 , }
};
}
