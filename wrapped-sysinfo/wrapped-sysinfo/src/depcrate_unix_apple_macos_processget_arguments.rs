// Generated macro for get_arguments (function)
macro_rules! Depcrate_unix_apple_macos_processget_arguments {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_arguments"}
// Dependencies: {}
fn get_arguments < 'a > (cmd : & mut Vec < OsString > , mut data : & 'a [u8] , mut n_args : c_int , refresh_cmd : bool ,) -> & 'a [u8] { if refresh_cmd { cmd . clear () ; } if n_args < 1 { return data ; } while data . first () == Some (& 0) { data = & data [1 ..] ; } while n_args > 0 && ! data . is_empty () { let pos = data . iter () . position (| c | * c == 0) . unwrap_or (data . len ()) ; let arg = & data [.. pos] ; if ! arg . is_empty () && refresh_cmd { cmd . push (OsStr :: from_bytes (arg) . to_os_string ()) ; } data = & data [pos ..] ; while data . first () == Some (& 0) { data = & data [1 ..] ; } n_args -= 1 ; } data }
};
}
