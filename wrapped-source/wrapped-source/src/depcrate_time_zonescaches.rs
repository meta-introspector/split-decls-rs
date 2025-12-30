// Generated macro for Caches (struct)
macro_rules! Depcrate_time_zonesCaches {
() => {
// Module: crate::time_zones
// Provides: {"Caches"}
// Dependencies: {}
# [derive (Debug , Default)] pub (crate) struct Caches { iana_to_bcp47 : Cache < BTreeMap < String , TimeZone > > , bcp47_to_canonical_iana : Cache < BTreeMap < TimeZone , String > > , primary_zones : Cache < BTreeMap < TimeZone , Region > > , metazones : Cache < MetazoneData > , }
};
}
