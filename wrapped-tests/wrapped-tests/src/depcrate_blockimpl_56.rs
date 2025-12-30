// Generated macro for impl_56 (impl)
macro_rules! Depcrate_blockimpl_56 {
() => {
// Module: crate::block
// Provides: {"impl_56"}
// Dependencies: {}
impl Count { fn current () -> Self { COUNT . with_borrow (| data | data . clone ()) } # [track_caller] fn assert_current (& self) { let current = Self :: current () ; if current != * self { panic ! ("got differing amounts of calls:
       current: `{current:?}`,
      expected: `{self:?}`") } } }
};
}
