// Generated macro for MetazoneData (struct)
macro_rules! Depcrate_time_zonesMetazoneData {
() => {
// Module: crate::time_zones
// Provides: {"MetazoneData"}
// Dependencies: {}
# [derive (Debug)] struct MetazoneData { periods : BTreeMap < TimeZone , Vec < (Timestamp , VariantOffsets , Option < MetazoneInfo >) > > , reverse : BTreeMap < (MetazoneId , MzMembership) , Vec < TimeZone > > , ids : BTreeMap < String , MetazoneId > , checksum : u64 , }
};
}
