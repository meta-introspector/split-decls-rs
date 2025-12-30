// Generated macro for bench_bulk_unbuffered (function)
macro_rules! Depcratebench_bulk_unbuffered {
() => {
// Module: crate
// Provides: {"bench_bulk_unbuffered"}
// Dependencies: {}
fn bench_bulk_unbuffered (client_config : Arc < ClientConfig > , server_config : Arc < ServerConfig > , plaintext_size : u64 , rounds : u64 ,) -> Timings { let server_name = "localhost" . try_into () . unwrap () ; let mut client = Unbuffered :: new_client (UnbufferedClientConnection :: new (client_config , server_name) . unwrap () ,) ; let mut server = Unbuffered :: new_server (UnbufferedServerConnection :: new (server_config) . unwrap ()) ; client . handshake (& mut server) ; let mut timings = Timings :: default () ; let buf = vec ! [0 ; plaintext_size as usize] ; for _ in 0 .. rounds { time (& mut timings . server , | | { server . write (& buf) ; }) ; server . swap_buffers (& mut client) ; time (& mut timings . client , | | { client . read_and_discard (buf . len ()) ; }) ; } timings }
};
}
