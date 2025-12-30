// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_time_zones_namesimpl_1244 {
() => {
// Module: crate::time_zones::names
// Provides: {"impl_1244"}
// Dependencies: {}
impl DataProvider < TimezoneIdentifiersIanaCoreV1 > for SourceDataProvider { fn load (& self , _ : DataRequest ,) -> Result < DataResponse < TimezoneIdentifiersIanaCoreV1 > , DataError > { let iana2bcp = self . iana_to_bcp47_map () ? ; let bcp2iana = self . bcp47_to_canonical_iana_map () ? ; let mut sorted_by_iana : Vec < (& TimeZone , & String) > = bcp2iana . iter () . collect () ; sorted_by_iana . sort_by_key (| & (_ , iana) | iana) ; let bcp47_ids = sorted_by_iana . iter () . map (| & (& tz , _) | tz) . collect () ; let bcp47_ids_checksum = compute_bcp47_ids_hash (& bcp47_ids) ; # [expect (clippy :: unwrap_used)] let map : BTreeMap < Vec < u8 > , usize > = iana2bcp . iter () . map (| (iana , bcp) | { let is_canonical = bcp2iana . get (bcp) == Some (iana) ; let index = bcp47_ids . iter () . position (| x | x == bcp) . unwrap () ; (if iana . contains ('/') { iana . to_owned () } else { format ! ("{}{iana}" , char :: from_u32 (icu :: time :: provider :: iana :: NON_REGION_CITY_PREFIX as u32) . unwrap ()) } . into_bytes () , (index << 1) | (is_canonical as usize) ,) }) . collect () ; let data_struct = IanaToBcp47Map { map : ZeroAsciiIgnoreCaseTrie :: try_from (& map) . map_err (| e | { DataError :: custom ("Could not create ZeroTrie from timezone.json data") . with_display_context (& e) }) ? . convert_store () , bcp47_ids : bcp47_ids . into () , } ; Ok (DataResponse { metadata : DataResponseMetadata :: default () . with_checksum (bcp47_ids_checksum) , payload : DataPayload :: from_owned (data_struct) , }) } }
};
}
