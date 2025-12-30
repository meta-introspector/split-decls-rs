// Generated macro for listen (function)
macro_rules! Depcratelisten {
() => {
// Module: crate
// Provides: {"listen"}
// Dependencies: {}
# [doc = " Starts listening for inbound QUIC connections on the given `sockets`."] # [doc = ""] # [doc = " Each socket is converted into a [`QuicListener`] with defaulted socket"] # [doc = " parameters. The listeners are then passed to [`listen_with_capabilities`]."] pub fn listen < S , M > (sockets : impl IntoIterator < Item = S > , params : ConnectionParams , cid_generator : impl ConnectionIdGenerator < 'static > + Clone , metrics : M ,) -> io :: Result < Vec < QuicConnectionStream < M > > > where S : TryInto < QuicListener , Error = io :: Error > , M : Metrics , { let quic_sockets : Vec < QuicListener > = sockets . into_iter () . map (| s | { # [cfg_attr (not (target_os = "linux") , expect (unused_mut))] let mut socket = s . try_into () ? ; # [cfg (target_os = "linux")] socket . apply_max_capabilities () ; Ok (socket) }) . collect :: < io :: Result < _ > > () ? ; listen_with_capabilities (quic_sockets , params , cid_generator , metrics) }
};
}
