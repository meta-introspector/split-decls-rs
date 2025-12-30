// Generated macro for test_book (function)
macro_rules! Depcratetest_book {
() => {
// Module: crate
// Provides: {"test_book"}
// Dependencies: {}
fn test_book () { println ! ("🧪 book") ; do_test (| | run_command ("cargo" , & ["clean"] , None , & []) , "book") ; do_test (| | run_command ("cargo" , & ["clean"] , Some ("firmware") , & []) , "book") ; do_test (| | { run_command ("cargo" , & ["build" , "-p" , "defmt" , "-p" , "defmt-decoder" , "--features" , "unstable-test" ,] , None , & [] ,) } , "book" ,) ; do_test (| | run_command ("cargo" , & ["build" , "-p" , "cortex-m"] , Some ("firmware") , & []) , "book" ,) ; do_test (| | { run_command ("mdbook" , & ["test" , "-L" , "../target/debug" , "-L" , "../target/debug/deps" , "-L" , "../firmware/target/debug" , "-L" , "../firmware/target/debug/deps" ,] , Some ("book") , & [("CARGO_CRATE_NAME" , "krate")] ,) } , "book" ,) ; }
};
}
