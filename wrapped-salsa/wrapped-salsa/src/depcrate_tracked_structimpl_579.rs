// Generated macro for impl_579 (impl)
macro_rules! Depcrate_tracked_structimpl_579 {
() => {
// Module: crate::tracked_struct
// Provides: {"impl_579"}
// Dependencies: {}
unsafe impl < C > Slot for Value < C > where C : Configuration , { # [inline (always)] unsafe fn memos (& self , current_revision : Revision) -> & crate :: table :: memo :: MemoTable { self . read_lock (current_revision) ; & self . memos } # [inline (always)] fn memos_mut (& mut self) -> & mut crate :: table :: memo :: MemoTable { & mut self . memos } }
};
}
