// Generated macro for new (function)
macro_rules! Depcratenew {
() => {
// Module: crate
// Provides: {"new"}
// Dependencies: {}
# [doc = " Create a new `want` channel."] pub fn new () -> (Giver , Taker) { let inner = Arc :: new (Inner { state : AtomicUsize :: new (State :: Idle . into ()) , task : TryLock :: new (None) , }) ; let inner2 = inner . clone () ; (Giver { inner , } , Taker { inner : inner2 , } ,) }
};
}
