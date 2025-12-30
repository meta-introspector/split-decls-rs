// Generated macro for impl_121 (impl)
macro_rules! Depcrate_elaborate_dropimpl_121 {
() => {
// Module: crate::elaborate_drop
// Provides: {"impl_121"}
// Dependencies: {}
impl Unwind { fn is_cleanup (self) -> bool { match self { Unwind :: To (..) => false , Unwind :: InCleanup => true , } } fn into_action (self) -> UnwindAction { match self { Unwind :: To (bb) => UnwindAction :: Cleanup (bb) , Unwind :: InCleanup => UnwindAction :: Terminate (UnwindTerminateReason :: InCleanup) , } } fn map < F > (self , f : F) -> Self where F : FnOnce (BasicBlock) -> BasicBlock , { match self { Unwind :: To (bb) => Unwind :: To (f (bb)) , Unwind :: InCleanup => Unwind :: InCleanup , } } }
};
}
