// Generated macro for fs_test_fn (function)
macro_rules! Depcratefs_test_fn {
() => {
// Module: crate
// Provides: {"fs_test_fn"}
// Dependencies: {}
pub fn fs_test_fn (count : usize , suffix : & str) { init () ; info ! ("(fs) Start {}" , count) ; let mut pathbuf = get_fs_path () ; pathbuf . push (format ! ("serial-test-test_{suffix}")) ; fs :: write (pathbuf . as_path () , count . to_ne_bytes ()) . unwrap () ; thread :: sleep (Duration :: from_millis (1000 * (count as u64))) ; info ! ("(fs) End {}" , count) ; let loaded = fs :: read (pathbuf . as_path ()) . map (| bytes | usize :: from_ne_bytes (bytes . as_slice () . try_into () . unwrap ())) . unwrap () ; assert_eq ! (loaded , count) ; }
};
}
