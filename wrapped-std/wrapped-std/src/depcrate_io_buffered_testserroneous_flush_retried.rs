// Generated macro for erroneous_flush_retried (function)
macro_rules! Depcrate_io_buffered_testserroneous_flush_retried {
() => {
// Module: crate::io::buffered::tests
// Provides: {"erroneous_flush_retried"}
// Dependencies: {}
# [doc = " Previously the `LineWriter` could successfully write some bytes but"] # [doc = " then fail to report that it has done so. Additionally, an erroneous"] # [doc = " flush after a successful write was permanently ignored."] # [doc = ""] # [doc = " Test that a line writer correctly reports the number of written bytes,"] # [doc = " and that it attempts to flush buffered lines from previous writes"] # [doc = " before processing new data"] # [doc = ""] # [doc = " Regression test for #37807"] # [test] fn erroneous_flush_retried () { let writer = ProgrammableSink { accept_prefix : Some (4) , max_writes : Some (2) , error_after_max_writes : true , .. Default :: default () } ; let mut writer = LineWriter :: new (writer) ; assert_eq ! (writer . write (b"a\nb\nc\nd\ne") . unwrap () , 8) ; assert_eq ! (writer . write (b"e") . unwrap () , 1) ; assert_eq ! (& writer . get_ref () . buffer , b"a\nb\nc\nd\n") ; }
};
}
