// Generated macro for get_exe (function)
macro_rules! Depcrate_unix_apple_macos_processget_exe {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_exe"}
// Dependencies: {}
fn get_exe (data : & [u8]) -> (& Path , & [u8]) { let pos = data . iter () . position (| c | * c == 0) . unwrap_or (data . len ()) ; let (exe , proc_args) = data . split_at (pos) ; (Path :: new (OsStr :: from_bytes (exe)) , proc_args) }
};
}
