// Generated macro for MANUAL_TESTS (const)
macro_rules! Depcrate_load_store_testsMANUAL_TESTS {
() => {
// Module: crate::load_store_tests
// Provides: {"MANUAL_TESTS"}
// Dependencies: {}
const MANUAL_TESTS : & str = "#[simd_test(enable = \"sve\")]
unsafe fn test_ffr() {
    svsetffr();
    let ffr = svrdffr();
    assert_vector_matches_u8(svdup_n_u8_z(ffr, 1), svindex_u8(1, 0));
    let pred = svdupq_n_b8(true, false, true, false, true, false, true, false,
                           true, false, true, false, true, false, true, false);
    svwrffr(pred);
    let ffr = svrdffr_z(svptrue_b8());
    assert_vector_matches_u8(svdup_n_u8_z(ffr, 1), svdup_n_u8_z(pred, 1));
}
" ;
};
}
