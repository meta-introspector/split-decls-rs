// Generated macro for test_poll (function)
macro_rules! Depcrate_ci_qemutest_poll {
() => {
// Module: crate::ci::qemu
// Provides: {"test_poll"}
// Dependencies: {}
fn test_poll (guest_ip : IpAddr) -> Result < () > { thread :: sleep (Duration :: from_secs (10)) ; let buf = "exit" ; let socket_addr = SocketAddr :: new (guest_ip , 9975) ; eprintln ! ("[CI] send {buf:?} via TCP to {socket_addr}") ; let mut stream = TcpStream :: connect (socket_addr) ? ; stream . write_all (buf . as_bytes ()) ? ; let mut buf = vec ! [] ; let received = stream . read_to_end (& mut buf) ? ; eprintln ! ("[CI] receive: {}" , from_utf8 (& buf [.. received]) ?) ; Ok (()) }
};
}
