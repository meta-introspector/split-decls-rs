// Generated macro for test_testudp (function)
macro_rules! Depcrate_ci_qemutest_testudp {
() => {
// Module: crate::ci::qemu
// Provides: {"test_testudp"}
// Dependencies: {}
fn test_testudp (guest_ip : IpAddr) -> Result < () > { thread :: sleep (Duration :: from_secs (10)) ; let buf = "exit" ; let socket_addr = SocketAddr :: new (guest_ip , 9975) ; eprintln ! ("[CI] send {buf:?} via UDP to {socket_addr}") ; let socket = UdpSocket :: bind ((Ipv4Addr :: UNSPECIFIED , 0)) ? ; socket . connect (socket_addr) ? ; socket . send (buf . as_bytes ()) ? ; Ok (()) }
};
}
