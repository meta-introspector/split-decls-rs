// Generated macro for Verify (struct)
macro_rules! DepcrateVerify {
() => {
// Module: crate
// Provides: {"Verify"}
// Dependencies: {}
# [doc = " Helper structure to package up arguments when sent to language-specific"] # [doc = " compilation backends for `LanguageMethods::verify`"] struct Verify < 'a > { wit_test : & 'a Path , bindings_dir : & 'a Path , artifacts_dir : & 'a Path , args : & 'a [String] , world : & 'a str , }
};
}
