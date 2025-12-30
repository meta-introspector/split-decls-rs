// Generated macro for test_read_fuzz_corpus (function)
macro_rules! Depcrate_msgs_message_testtest_read_fuzz_corpus {
() => {
// Module: crate::msgs::message_test
// Provides: {"test_read_fuzz_corpus"}
// Dependencies: {}
# [test] fn test_read_fuzz_corpus () { fn corpus_dir () -> PathBuf { let from_subcrate = Path :: new ("../fuzz/corpus/message") ; let from_root = Path :: new ("fuzz/corpus/message") ; if from_root . is_dir () { from_root . to_path_buf () } else { from_subcrate . to_path_buf () } } for file in fs :: read_dir (corpus_dir ()) . unwrap () { let mut f = fs :: File :: open (file . unwrap () . path ()) . unwrap () ; let mut bytes = Vec :: new () ; f . read_to_end (& mut bytes) . unwrap () ; let mut rd = Reader :: init (& bytes) ; let msg = PlainMessage :: read (& mut rd) . unwrap () ; println ! ("{msg:?}") ; let Ok (msg) = Message :: try_from (msg) else { continue ; } ; let enc = PlainMessage :: from (msg) . into_unencrypted_opaque () . encode () ; assert_eq ! (bytes . to_vec () , enc) ; assert_eq ! (bytes [.. rd . used ()] . to_vec () , enc) ; } }
};
}
