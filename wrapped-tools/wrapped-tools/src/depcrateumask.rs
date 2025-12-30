// Generated macro for umask (function)
macro_rules! Depcrateumask {
() => {
// Module: crate
// Provides: {"umask"}
// Dependencies: {}
# [doc = " Get the umask in a way that is safe, but may be too slow for use outside of tests."] # [cfg (unix)] pub fn umask () -> u32 { let output = std :: process :: Command :: new ("/bin/sh") . args (["-c" , "umask"]) . output () . expect ("can execute `sh -c umask`") ; assert ! (output . status . success () , "`sh -c umask` failed") ; assert_eq ! (output . stderr . as_bstr () , "" , "`sh -c umask` unexpected message") ; let text = output . stdout . to_str () . expect ("valid Unicode") . trim () ; u32 :: from_str_radix (text , 8) . expect ("parses as octal number") }
};
}
