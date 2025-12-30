// Generated macro for remove_dir_all (function)
macro_rules! Depcrateremove_dir_all {
() => {
// Module: crate
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [cfg (windows)] fn remove_dir_all (path : & Path) -> io :: Result < () > { for _ in 0 .. 99 { if fs :: remove_dir_all (path) . is_ok () { return Ok (()) ; } std :: thread :: sleep (std :: time :: Duration :: from_millis (10)) } fs :: remove_dir_all (path) }
};
}
