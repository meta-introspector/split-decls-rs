// Generated macro for version_works (function)
macro_rules! Depcrateversion_works {
() => {
// Module: crate
// Provides: {"version_works"}
// Dependencies: {}
# [test] fn version_works () { unsafe { println ! ("{:#x}" , OpenSSL_version_num ()) ; assert ! (OpenSSL_version_num () > 0) ; } }
};
}
