// Generated macro for CentralDirectoryEndInfo (struct)
macro_rules! Depcrate_specCentralDirectoryEndInfo {
() => {
// Module: crate::spec
// Provides: {"CentralDirectoryEndInfo"}
// Dependencies: {}
pub (crate) struct CentralDirectoryEndInfo { pub eocd : DataAndPosition < Zip32CentralDirectoryEnd > , pub eocd64 : Option < DataAndPosition < Zip64CentralDirectoryEnd > > , pub archive_offset : u64 , }
};
}
