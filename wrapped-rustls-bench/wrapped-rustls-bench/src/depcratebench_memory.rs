// Generated macro for bench_memory (function)
macro_rules! Depcratebench_memory {
() => {
// Module: crate
// Provides: {"bench_memory"}
// Dependencies: {}
fn bench_memory (client_config : Arc < ClientConfig > , server_config : Arc < ServerConfig > , conn_count : u64 ,) { let conn_count = (conn_count / 2) as usize ; let mut servers = Vec :: with_capacity (conn_count) ; let mut clients = Vec :: with_capacity (conn_count) ; let mut buffers = TempBuffers :: new () ; for _i in 0 .. conn_count { servers . push (ServerConnection :: new (server_config . clone ()) . unwrap ()) ; let server_name = "localhost" . try_into () . unwrap () ; clients . push (ClientConnection :: new (client_config . clone () , server_name) . unwrap ()) ; } for _step in 0 .. 5 { for (client , server) in clients . iter_mut () . zip (servers . iter_mut ()) { do_handshake_step (& mut buffers , client , server) ; } } for client in clients . iter_mut () { client . writer () . write_all (& [0u8 ; 1024]) . unwrap () ; } for (client , server) in clients . iter_mut () . zip (servers . iter_mut ()) { transfer (& mut buffers , client , server , Some (1024)) ; } }
};
}
