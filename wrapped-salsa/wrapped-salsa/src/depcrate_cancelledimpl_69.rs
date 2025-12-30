// Generated macro for impl_69 (impl)
macro_rules! Depcrate_cancelledimpl_69 {
() => {
// Module: crate::cancelled
// Provides: {"impl_69"}
// Dependencies: {}
impl std :: fmt :: Display for Cancelled { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let why = match self { Cancelled :: PendingWrite => "pending write" , Cancelled :: PropagatedPanic => "propagated panic" , } ; f . write_str ("cancelled because of ") ? ; f . write_str (why) } }
};
}
