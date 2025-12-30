// Generated macro for impl_328 (impl)
macro_rules! Depcrate_internedimpl_328 {
() => {
// Module: crate::interned
// Provides: {"impl_328"}
// Dependencies: {}
unsafe impl < C > Slot for Value < C > where C : Configuration , { # [inline (always)] unsafe fn memos (& self , _current_revision : Revision) -> & MemoTable { unsafe { & * self . memos . get () } } # [inline (always)] fn memos_mut (& mut self) -> & mut MemoTable { self . memos . get_mut () } }
};
}
