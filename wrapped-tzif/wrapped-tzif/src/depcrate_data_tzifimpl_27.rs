// Generated macro for impl_27 (impl)
macro_rules! Depcrate_data_tzifimpl_27 {
() => {
// Module: crate::data::tzif
// Provides: {"impl_27"}
// Dependencies: {}
impl TzifHeader { # [doc = " Returns the version number of the `TZif` header."] pub fn version (& self) -> usize { self . version } # [doc = " Returns the number of bytes per time object based on the version number."] pub fn time_size < const V : usize > () -> usize { match V { 1 => 4 , _ => 8 , } } # [doc = " Returns the exact size of the data block in bytes based on the header."] pub fn block_size < const V : usize > (& self) -> usize { let time_size = Self :: time_size :: < V > () ; self . timecnt * time_size + self . timecnt + self . typecnt * 6 + self . charcnt + self . leapcnt * (time_size + 4) + self . isstdcnt + self . isutcnt } }
};
}
