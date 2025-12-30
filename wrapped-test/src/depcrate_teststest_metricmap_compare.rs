// Generated macro for test_metricmap_compare (function)
macro_rules! Depcrate_teststest_metricmap_compare {
() => {
// Module: crate::tests
// Provides: {"test_metricmap_compare"}
// Dependencies: {}
# [test] fn test_metricmap_compare () { let mut m1 = MetricMap :: new () ; let mut m2 = MetricMap :: new () ; m1 . insert_metric ("in-both-noise" , 1000.0 , 200.0) ; m2 . insert_metric ("in-both-noise" , 1100.0 , 200.0) ; m1 . insert_metric ("in-first-noise" , 1000.0 , 2.0) ; m2 . insert_metric ("in-second-noise" , 1000.0 , 2.0) ; m1 . insert_metric ("in-both-want-downwards-but-regressed" , 1000.0 , 10.0) ; m2 . insert_metric ("in-both-want-downwards-but-regressed" , 2000.0 , 10.0) ; m1 . insert_metric ("in-both-want-downwards-and-improved" , 2000.0 , 10.0) ; m2 . insert_metric ("in-both-want-downwards-and-improved" , 1000.0 , 10.0) ; m1 . insert_metric ("in-both-want-upwards-but-regressed" , 2000.0 , - 10.0) ; m2 . insert_metric ("in-both-want-upwards-but-regressed" , 1000.0 , - 10.0) ; m1 . insert_metric ("in-both-want-upwards-and-improved" , 1000.0 , - 10.0) ; m2 . insert_metric ("in-both-want-upwards-and-improved" , 2000.0 , - 10.0) ; }
};
}
