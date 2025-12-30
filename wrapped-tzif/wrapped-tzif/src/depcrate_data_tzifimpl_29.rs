// Generated macro for impl_29 (impl)
macro_rules! Depcrate_data_tzifimpl_29 {
() => {
// Module: crate::data::tzif
// Provides: {"impl_29"}
// Dependencies: {}
impl TzifData { # [doc = " Returns the version number of this `TZif` data."] pub fn version_number (& self) -> usize { self . header2 . as_ref () . map_or (self . header1 . version () , TzifHeader :: version) } # [doc = " Returns the number of bytes per time object based on the version number."] pub fn time_size (& self) -> usize { match self . version_number () { 1 => 4 , _ => 8 , } } # [doc = " Returns the exact size of the data block in bytes based on the header."] pub fn block_size < const V : usize > (& self) -> Option < usize > { match V { 1 => Some (self . header1 . block_size :: < V > ()) , _ => self . header2 . as_ref () . map (TzifHeader :: block_size :: < V >) , } } }
};
}
