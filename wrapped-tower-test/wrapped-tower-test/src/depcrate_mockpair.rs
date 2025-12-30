// Generated macro for pair (function)
macro_rules! Depcrate_mockpair {
() => {
// Module: crate::mock
// Provides: {"pair"}
// Dependencies: {}
# [doc = " Create a new `Mock` and `Handle` pair."] pub fn pair < T , U > () -> (Mock < T , U > , Handle < T , U >) { let (tx , rx) = mpsc :: unbounded_channel () ; let tx = Mutex :: new (tx) ; let state = Arc :: new (Mutex :: new (State :: new ())) ; let mock = Mock { id : 0 , tx , state : state . clone () , can_send : false , } ; let handle = Handle { rx , state } ; (mock , handle) }
};
}
