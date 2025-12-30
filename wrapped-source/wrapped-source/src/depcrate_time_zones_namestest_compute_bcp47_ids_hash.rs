// Generated macro for test_compute_bcp47_ids_hash (function)
macro_rules! Depcrate_time_zones_namestest_compute_bcp47_ids_hash {
() => {
// Module: crate::time_zones::names
// Provides: {"test_compute_bcp47_ids_hash"}
// Dependencies: {}
# [test] fn test_compute_bcp47_ids_hash () { use icu :: locale :: subtags :: subtag ; let bcp47_ids = vec ! [TimeZone (subtag ! ("aedxb")) , TimeZone (subtag ! ("brfor")) , TimeZone (subtag ! ("usinvev")) ,] ; let checksum1 = compute_bcp47_ids_hash (& bcp47_ids) ; assert_eq ! (checksum1 , 2080308884639987833) ; let bcp47_ids_rev = vec ! [TimeZone (subtag ! ("usinvev")) , TimeZone (subtag ! ("aedxb")) , TimeZone (subtag ! ("brfor")) ,] ; let checksum3 = compute_bcp47_ids_hash (& bcp47_ids_rev) ; assert_ne ! (checksum1 , checksum3) ; let bcp47_ids_roll = vec ! [TimeZone (subtag ! ("aedx")) , TimeZone (subtag ! ("bbrfor")) , TimeZone (subtag ! ("usinvev")) ,] ; let checksum4 = compute_bcp47_ids_hash (& bcp47_ids_roll) ; assert_ne ! (checksum1 , checksum4) ; assert_ne ! (checksum3 , checksum4) ; }
};
}
