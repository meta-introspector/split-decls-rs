// Generated macro for test_mioudp (function)
macro_rules! Depcrate_ci_qemutest_mioudp {
() => {
// Module: crate::ci::qemu
// Provides: {"test_mioudp"}
// Dependencies: {}
fn test_mioudp (guest_ip : IpAddr) -> Result < () > { thread :: sleep (Duration :: from_secs (10)) ; let buf = "exit" ; let socket_addr = SocketAddr :: new (guest_ip , 9975) ; eprintln ! ("[CI] send {buf:?} via UDP to {socket_addr}") ; let socket = UdpSocket :: bind ((Ipv4Addr :: UNSPECIFIED , 0)) ? ; socket . connect (socket_addr) ? ; socket . send (buf . as_bytes ()) ? ; socket . set_read_timeout (Some (Duration :: from_secs (10))) ? ; let mut buf = [0 ; 128] ; let received = socket . recv (& mut buf) ? ; eprintln ! ("[CI] receive: {}" , from_utf8 (& buf [.. received]) ?) ; Ok (()) }
};
}
