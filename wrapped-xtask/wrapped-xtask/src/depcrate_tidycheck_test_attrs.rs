// Generated macro for check_test_attrs (function)
macro_rules! Depcrate_tidycheck_test_attrs {
() => {
// Module: crate::tidy
// Provides: {"check_test_attrs"}
// Dependencies: {}
fn check_test_attrs (path : & Path , text : & str) { let panic_rule = "https://github.com/rust-lang/rust-analyzer/blob/master/docs/book/src/contributing/style.md#should_panic" ; let need_panic : & [& str] = & ["slow-tests/tidy.rs" , "test-utils/src/fixture.rs" , "ide-db/src/generated/lints.rs" ,] ; if need_panic . iter () . any (| p | path . ends_with (p)) { return ; } if let Some ((line , _)) = text . lines () . tuple_windows () . enumerate () . find (| (_ , (a , b)) | b . contains ("#[should_panic") && ! a . contains ("FIXME")) { panic ! ("\ndon't add `#[should_panic]` tests, see:\n\n    {}\n\n   {}:{line}\n" , panic_rule , path . display () ,) } }
};
}
