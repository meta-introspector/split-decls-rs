// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_time_zones_namesimpl_1246 {
() => {
// Module: crate::time_zones::names
// Provides: {"impl_1246"}
// Dependencies: {}
impl DataProvider < TimezoneIdentifiersIanaExtendedV1 > for SourceDataProvider { fn load (& self , _ : DataRequest ,) -> Result < DataResponse < TimezoneIdentifiersIanaExtendedV1 > , DataError > { let iana2bcp = self . iana_to_bcp47_map () ? ; let bcp2iana = self . bcp47_to_canonical_iana_map () ? ; let mut sorted_by_iana : Vec < (& TimeZone , & String) > = bcp2iana . iter () . collect () ; sorted_by_iana . sort_by_key (| & (_ , iana) | iana) ; let bcp47_ids_checksum = compute_bcp47_ids_hash (& sorted_by_iana . iter () . map (| & (& tz , _) | tz) . collect ()) ; let canonical_iana_ids = sorted_by_iana . iter () . map (| & (_ , iana) | iana) . collect :: < Vec < _ > > () ; let mut non_canonical_iana_ids = iana2bcp . keys () . filter (| k | ! canonical_iana_ids . contains (k)) . collect :: < Vec < _ > > () ; non_canonical_iana_ids . sort_by (| a , b | { a . as_bytes () . iter () . map (u8 :: to_ascii_lowercase) . cmp (b . as_bytes () . iter () . map (u8 :: to_ascii_lowercase)) }) ; let normalized_iana_ids = canonical_iana_ids . into_iter () . chain (non_canonical_iana_ids) . collect :: < Vec < _ > > () ; let data_struct = IanaNames { normalized_iana_ids : normalized_iana_ids . as_slice () . into () , } ; Ok (DataResponse { metadata : DataResponseMetadata :: default () . with_checksum (bcp47_ids_checksum) , payload : DataPayload :: from_owned (data_struct) , }) } }
};
}
