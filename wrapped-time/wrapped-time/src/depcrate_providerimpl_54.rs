// Generated macro for impl_54 (impl)
macro_rules! Depcrate_providerimpl_54 {
() => {
// Module: crate::provider
// Provides: {"impl_54"}
// Dependencies: {}
impl TimezonePeriods < '_ > { # [doc = " Gets the information for a time zone at at timestamp"] # [doc = ""] # [doc = " If the timezone is in a metazone, returns the metazone ID as well as the offsets"] # [doc = " that the metazone's golden zone currently uses."] pub fn get (& self , time_zone_id : TimeZone , timestamp : ZoneNameTimestamp ,) -> Option < (VariantOffsets , Option < MetazoneInfo >) > { let (os_idx , NichedOption (mz)) = self . find_period (self . index . get (time_zone_id . as_str ()) ? , timestamp) ? ; let os = self . offsets . get (os_idx as usize) ? ; let Some (mz) = mz else { return Some ((os . offsets , None)) ; } ; Some ((os . offsets , Some (MetazoneInfo { id : mz , kind : os . mzmsk , }) ,)) } fn find_period (& self , idx : usize , timestamp : ZoneNameTimestamp ,) -> Option < (u8 , NichedOption < MetazoneId , 1 >) > { use zerovec :: ule :: vartuple :: VarTupleULE ; use zerovec :: ule :: AsULE ; let & VarTupleULE { sized : first , variable : ref rest , } = self . list . get (idx) ? ; let i = match rest . binary_search_by (| (t , ..) | t . cmp (& Timestamp24 (timestamp))) { Err (0) => return Some (< (u8 , NichedOption < MetazoneId , 1 >) > :: from_unaligned (first)) , Err (i) => i - 1 , Ok (i) => i , } ; let (_ , os , mz) = rest . get (i) ? ; Some ((os , mz)) } }
};
}
