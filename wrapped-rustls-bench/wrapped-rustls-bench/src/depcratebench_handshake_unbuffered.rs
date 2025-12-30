// Generated macro for bench_handshake_unbuffered (function)
macro_rules! Depcratebench_handshake_unbuffered {
() => {
// Module: crate
// Provides: {"bench_handshake_unbuffered"}
// Dependencies: {}
fn bench_handshake_unbuffered (mut rounds : u64 , resume : ResumptionParam , client_config : Arc < ClientConfig > , server_config : Arc < ServerConfig > ,) -> Timings { let mut timings = Timings :: default () ; while rounds > 0 { let mut client_time = 0f64 ; let mut server_time = 0f64 ; let client = time (& mut client_time , | | { let server_name = "localhost" . try_into () . unwrap () ; UnbufferedClientConnection :: new (client_config . clone () , server_name) . unwrap () }) ; let server = time (& mut server_time , | | { UnbufferedServerConnection :: new (server_config . clone ()) . unwrap () }) ; let mut client = Unbuffered :: new_client (client) ; let mut server = Unbuffered :: new_server (server) ; let client_wrote = time (& mut client_time , | | client . communicate ()) ; if client_wrote { client . swap_buffers (& mut server) ; } let server_wrote = time (& mut server_time , | | server . communicate ()) ; if server_wrote { server . swap_buffers (& mut client) ; } let client_wrote = time (& mut client_time , | | client . communicate ()) ; if client_wrote { client . swap_buffers (& mut server) ; } let server_wrote = time (& mut server_time , | | server . communicate ()) ; if server_wrote { server . swap_buffers (& mut client) ; } assert ! (! server . communicate ()) ; assert ! (! client . communicate ()) ; if client . conn . handshake_kind () == Some (resume . as_handshake_kind ()) && server . conn . handshake_kind () == Some (resume . as_handshake_kind ()) { timings . client += client_time ; timings . server += server_time ; rounds -= 1 ; } else { } } timings }
};
}
