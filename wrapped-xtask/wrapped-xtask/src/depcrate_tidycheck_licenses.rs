// Generated macro for check_licenses (function)
macro_rules! Depcrate_tidycheck_licenses {
() => {
// Module: crate::tidy
// Provides: {"check_licenses"}
// Dependencies: {}
fn check_licenses (sh : & Shell) { const EXPECTED : & [& str] = & ["(MIT OR Apache-2.0) AND Unicode-3.0" , "0BSD OR MIT OR Apache-2.0" , "Apache-2.0 / MIT" , "Apache-2.0 OR BSL-1.0" , "Apache-2.0 OR MIT" , "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT" , "Apache-2.0 WITH LLVM-exception" , "Apache-2.0" , "Apache-2.0/MIT" , "BSD-2-Clause OR Apache-2.0 OR MIT" , "CC0-1.0" , "ISC" , "MIT / Apache-2.0" , "MIT OR Apache-2.0 OR LGPL-2.1-or-later" , "MIT OR Apache-2.0" , "MIT OR Zlib OR Apache-2.0" , "MIT" , "MIT/Apache-2.0" , "MPL-2.0" , "Unicode-3.0" , "Unlicense OR MIT" , "Unlicense/MIT" , "Zlib" ,] ; let meta = cmd ! (sh , "cargo metadata --format-version 1") . read () . unwrap () ; let mut licenses = meta . split ([',' , '{' , '}']) . filter (| it | it . contains (r#""license""#)) . map (| it | it . trim ()) . map (| it | it [r#""license":"# . len () ..] . trim_matches ('"')) . collect :: < Vec < _ > > () ; licenses . sort_unstable () ; licenses . dedup () ; let mut expected = EXPECTED . to_vec () ; expected . sort_unstable () ; if licenses != expected { let mut diff = String :: new () ; diff . push_str ("New Licenses:\n") ; for & l in licenses . iter () { if ! expected . contains (& l) { diff += & format ! ("  {l}\n") } } diff . push_str ("\nMissing Licenses:\n") ; for l in expected { if ! licenses . contains (& l) { diff += & format ! ("  {l}\n") } } panic ! ("different set of licenses!\n{diff}") ; } assert_eq ! (licenses , expected) ; }
};
}
