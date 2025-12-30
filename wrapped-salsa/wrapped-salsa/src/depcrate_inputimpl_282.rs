// Generated macro for impl_282 (impl)
macro_rules! Depcrate_inputimpl_282 {
() => {
// Module: crate::input
// Provides: {"impl_282"}
// Dependencies: {}
unsafe impl < C > Slot for Value < C > where C : Configuration , { # [inline (always)] unsafe fn memos (& self , _current_revision : Revision) -> & crate :: table :: memo :: MemoTable { & self . memos } # [inline (always)] fn memos_mut (& mut self) -> & mut crate :: table :: memo :: MemoTable { & mut self . memos } }
};
}
