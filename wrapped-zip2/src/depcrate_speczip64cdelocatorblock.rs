// Generated macro for Zip64CDELocatorBlock (struct)
macro_rules! Depcrate_specZip64CDELocatorBlock {
() => {
// Module: crate::spec
// Provides: {"Zip64CDELocatorBlock"}
// Dependencies: {}
# [derive (Copy , Clone)] # [repr (packed , C)] pub (crate) struct Zip64CDELocatorBlock { magic : Magic , pub disk_with_central_directory : u32 , pub end_of_central_directory_offset : u64 , pub number_of_disks : u32 , }
};
}
