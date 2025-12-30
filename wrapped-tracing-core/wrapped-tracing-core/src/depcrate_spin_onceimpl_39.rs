// Generated macro for impl_39 (impl)
macro_rules! Depcrate_spin_onceimpl_39 {
() => {
// Module: crate::spin::once
// Provides: {"impl_39"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Once < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . r#try () { Some (s) => write ! (f , "Once {{ data: ") . and_then (| () | s . fmt (f)) . and_then (| () | write ! (f , "}}")) , None => write ! (f , "Once {{ <uninitialized> }}") , } } }
};
}
