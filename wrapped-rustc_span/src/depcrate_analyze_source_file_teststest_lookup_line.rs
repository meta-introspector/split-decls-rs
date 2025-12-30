// Generated macro for test_lookup_line (function)
macro_rules! Depcrate_analyze_source_file_teststest_lookup_line {
() => {
// Module: crate::analyze_source_file::tests
// Provides: {"test_lookup_line"}
// Dependencies: {}
# [test] fn test_lookup_line () { let source = "abcdefghijklm\nabcdefghij\n..." . to_owned () ; let mut sf = SourceFile :: new (FileName :: Anon (Hash64 :: ZERO) , source , SourceFileHashAlgorithm :: Sha256 , Some (SourceFileHashAlgorithm :: Sha256) ,) . unwrap () ; sf . start_pos = BytePos (3) ; assert_eq ! (sf . lines () , & [RelativeBytePos (0) , RelativeBytePos (14) , RelativeBytePos (25)]) ; assert_eq ! (sf . lookup_line (RelativeBytePos (0)) , Some (0)) ; assert_eq ! (sf . lookup_line (RelativeBytePos (1)) , Some (0)) ; assert_eq ! (sf . lookup_line (RelativeBytePos (13)) , Some (0)) ; assert_eq ! (sf . lookup_line (RelativeBytePos (14)) , Some (1)) ; assert_eq ! (sf . lookup_line (RelativeBytePos (15)) , Some (1)) ; assert_eq ! (sf . lookup_line (RelativeBytePos (25)) , Some (2)) ; assert_eq ! (sf . lookup_line (RelativeBytePos (26)) , Some (2)) ; }
};
}
