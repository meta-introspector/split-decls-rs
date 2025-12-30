// Generated macro for get_environ (function)
macro_rules! Depcrate_unix_apple_macos_processget_environ {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_environ"}
// Dependencies: {}
fn get_environ (environ : & mut Vec < OsString > , mut data : & [u8]) { environ . clear () ; while data . first () == Some (& 0) { data = & data [1 ..] ; } while ! data . is_empty () { let pos = data . iter () . position (| c | * c == 0) . unwrap_or (data . len ()) ; let arg = & data [.. pos] ; if arg . is_empty () { return ; } environ . push (OsStr :: from_bytes (arg) . to_os_string ()) ; data = & data [pos ..] ; while data . first () == Some (& 0) { data = & data [1 ..] ; } } }
};
}
