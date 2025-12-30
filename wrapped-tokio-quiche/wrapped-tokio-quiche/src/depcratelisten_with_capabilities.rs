// Generated macro for listen_with_capabilities (function)
macro_rules! Depcratelisten_with_capabilities {
() => {
// Module: crate
// Provides: {"listen_with_capabilities"}
// Dependencies: {}
# [doc = " Starts listening for inbound QUIC connections on the given"] # [doc = " [`QuicListener`]s."] # [doc = ""] # [doc = " Each socket starts a separate tokio task to process and route inbound"] # [doc = " packets. This task emits connections on the respective"] # [doc = " [`QuicConnectionStream`] after receiving the client's QUIC initial and"] # [doc = " (optionally) validating its IP address."] # [doc = ""] # [doc = " The task shuts down when the returned stream is closed (or dropped) and all"] # [doc = " previously-yielded connections are closed."] pub fn listen_with_capabilities < M > (sockets : impl IntoIterator < Item = QuicListener > , params : ConnectionParams , cid_generator : impl ConnectionIdGenerator < 'static > + Clone , metrics : M ,) -> io :: Result < Vec < QuicConnectionStream < M > > > where M : Metrics , { if params . settings . capture_quiche_logs { capture_quiche_logs () ; } sockets . into_iter () . map (| s | { crate :: quic :: start_listener (s , & params , cid_generator . clone () , metrics . clone () ,) }) . collect () }
};
}
