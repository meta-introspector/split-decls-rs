// Generated macro for Client (struct)
macro_rules! Depcrate_jobserverClient {
() => {
// Module: crate::jobserver
// Provides: {"Client"}
// Dependencies: {}
# [derive (Clone)] pub struct Client { helper : Option < Arc < jobserver :: HelperThread > > , tx : Option < mpsc :: UnboundedSender < oneshot :: Sender < io :: Result < jobserver :: Acquired > > > > , inner : jobserver :: Client , }
};
}
