// Generated macro for ConnStream (type)
macro_rules! Depcrate_quic_routerConnStream {
() => {
// Module: crate::quic::router
// Provides: {"ConnStream"}
// Dependencies: {}
type ConnStream < Tx , M > = mpsc :: Receiver < io :: Result < InitialQuicConnection < Tx , M > > > ;
};
}
