// Generated macro for impl_576 (impl)
macro_rules! Depcrate_tracked_structimpl_576 {
() => {
// Module: crate::tracked_struct
// Provides: {"impl_576"}
// Dependencies: {}
unsafe impl < C > Slot for Value < C > where C : Configuration , { # [inline (always)] unsafe fn memos (& self , current_revision : Revision) -> & crate :: table :: memo :: MemoTable { self . read_lock (current_revision) ; & self . memos } # [inline (always)] fn memos_mut (& mut self) -> & mut crate :: table :: memo :: MemoTable { & mut self . memos } }
};
}
