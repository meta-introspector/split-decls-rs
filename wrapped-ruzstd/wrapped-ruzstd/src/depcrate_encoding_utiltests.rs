// Generated macro for tests (module)
macro_rules! Depcrate_encoding_utiltests {
() => {
// Module: crate::encoding::util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: find_min_size ; use super :: minify_val ; use alloc :: vec ; # [test] fn min_size_detection () { assert_eq ! (find_min_size (0) , 1) ; assert_eq ! (find_min_size (0xff) , 1) ; assert_eq ! (find_min_size (0xff_ff) , 2) ; assert_eq ! (find_min_size (0x00_ff_ff_ff) , 4) ; assert_eq ! (find_min_size (0xff_ff_ff_ff) , 4) ; assert_eq ! (find_min_size (0x00ff_ffff_ffff_ffff) , 8) ; assert_eq ! (find_min_size (0xffff_ffff_ffff_ffff) , 8) ; } # [test] fn bytes_minified () { assert_eq ! (minify_val (0) , vec ! [0]) ; assert_eq ! (minify_val (0xff) , vec ! [0xff]) ; assert_eq ! (minify_val (0xff_ff) , vec ! [0xff , 0xff]) ; assert_eq ! (minify_val (0xff_ff_ff_ff) , vec ! [0xff , 0xff , 0xff , 0xff]) ; assert_eq ! (minify_val (0xffff_ffff_ffff_ffff) , vec ! [0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff , 0xff]) ; } }
};
}
