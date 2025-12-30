// Generated macro for test_check_for_field (function)
macro_rules! Depcrate_datetime_neo_skeletontest_check_for_field {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"test_check_for_field"}
// Dependencies: {}
# [test] fn test_check_for_field () { assert ! (check_for_field (DataMarkerAttributes :: from_str_or_panic ("ym0d") , "y")) ; assert ! (check_for_field (DataMarkerAttributes :: from_str_or_panic ("ym0d") , "m0")) ; assert ! (check_for_field (DataMarkerAttributes :: from_str_or_panic ("ym0d") , "d")) ; assert ! (! check_for_field (DataMarkerAttributes :: from_str_or_panic ("ym0d") , "y0")) ; assert ! (! check_for_field (DataMarkerAttributes :: from_str_or_panic ("ym0d") , "m")) ; assert ! (check_for_field (DataMarkerAttributes :: from_str_or_panic ("eh0") , "e")) ; assert ! (check_for_field (DataMarkerAttributes :: from_str_or_panic ("eh0") , "h0")) ; assert ! (! check_for_field (DataMarkerAttributes :: from_str_or_panic ("eh0") , "e0")) ; assert ! (! check_for_field (DataMarkerAttributes :: from_str_or_panic ("eh0") , "h")) ; }
};
}
