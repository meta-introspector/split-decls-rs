// Generated macro for ZeroCopyCheckExporter (struct)
macro_rules! Depcrate_tests_make_testdataZeroCopyCheckExporter {
() => {
// Module: crate::tests::make_testdata
// Provides: {"ZeroCopyCheckExporter"}
// Dependencies: {}
struct ZeroCopyCheckExporter { zero_copy_violations : Mutex < BTreeSet < DataMarkerInfo > > , zero_copy_transient_violations : Mutex < BTreeSet < DataMarkerInfo > > , rountrip_errors : Mutex < BTreeSet < (DataMarkerInfo , String) > > , }
};
}
