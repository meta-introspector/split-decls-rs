// Generated macro for impl_28 (impl)
macro_rules! Depcrate_spin_muteximpl_28 {
() => {
// Module: crate::spin::mutex
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Debug > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_lock () { Some (guard) => write ! (f , "Mutex {{ data: ") . and_then (| () | (& * guard) . fmt (f)) . and_then (| () | write ! (f , "}}")) , None => write ! (f , "Mutex {{ <locked> }}") , } } }
};
}
