// Generated macro for bench_bulk_buffered (function)
macro_rules! Depcratebench_bulk_buffered {
() => {
// Module: crate
// Provides: {"bench_bulk_buffered"}
// Dependencies: {}
fn bench_bulk_buffered (client_config : Arc < ClientConfig > , server_config : Arc < ServerConfig > , plaintext_size : u64 , rounds : u64 ,) -> Timings { let server_name = "localhost" . try_into () . unwrap () ; let mut client = ClientConnection :: new (client_config , server_name) . unwrap () ; client . set_buffer_limit (None) ; let mut server = ServerConnection :: new (server_config) . unwrap () ; server . set_buffer_limit (None) ; let mut timings = Timings :: default () ; let mut buffers = TempBuffers :: new () ; do_handshake (& mut buffers , & mut client , & mut server) ; let buf = vec ! [0 ; plaintext_size as usize] ; for _ in 0 .. rounds { time (& mut timings . server , | | { server . writer () . write_all (& buf) . unwrap () ; }) ; timings . client += transfer (& mut buffers , & mut server , & mut client , Some (buf . len ())) ; } timings }
};
}
