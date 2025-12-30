// Generated macro for validate (function)
macro_rules! Depcrate_samplesvalidate {
() => {
// Module: crate::samples
// Provides: {"validate"}
// Dependencies: {}
# [test] fn validate () { use crate :: { VarZeroVec , ZeroVec } ; assert_eq ! (ZeroVec ::< u32 >:: parse_bytes (TEST_BUFFER_LE) . unwrap () , ZeroVec :: alloc_from_slice (TEST_SLICE)) ; assert_eq ! (TEST_SLICE . iter () . sum ::< u32 > () , TEST_SUM) ; assert_eq ! (serde_json :: from_str ::< ZeroVec ::< u32 >> (JSON_STR) . unwrap () , ZeroVec :: alloc_from_slice (TEST_SLICE)) ; assert_eq ! (bincode :: deserialize ::< ZeroVec ::< u32 >> (BINCODE_BUF) . unwrap () , ZeroVec :: alloc_from_slice (TEST_SLICE)) ; VarZeroVec :: < str > :: parse_bytes (TEST_VARZEROSLICE_BYTES) . unwrap () ; }
};
}
