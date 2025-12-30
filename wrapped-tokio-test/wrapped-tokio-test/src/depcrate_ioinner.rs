// Generated macro for Inner (struct)
macro_rules! Depcrate_ioInner {
() => {
// Module: crate::io
// Provides: {"Inner"}
// Dependencies: {}
struct Inner { actions : VecDeque < Action > , waiting : Option < Instant > , sleep : Option < Pin < Box < Sleep > > > , read_wait : Option < Waker > , rx : UnboundedReceiverStream < Action > , name : String , }
};
}
