// Generated macro for is_lfs_pointer_file (function)
macro_rules! Depcrateis_lfs_pointer_file {
() => {
// Module: crate
// Provides: {"is_lfs_pointer_file"}
// Dependencies: {}
fn is_lfs_pointer_file (path : & Path) -> bool { const PREFIX : & [u8] = b"version https://git-lfs" ; let mut buf = [0_u8 ; PREFIX . len ()] ; std :: fs :: OpenOptions :: new () . read (true) . open (path) . is_ok_and (| mut f | f . read_exact (& mut buf) . is_ok_and (| _ | buf . starts_with (PREFIX))) }
};
}
