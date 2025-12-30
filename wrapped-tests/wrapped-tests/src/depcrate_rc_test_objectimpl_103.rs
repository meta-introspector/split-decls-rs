// Generated macro for impl_103 (impl)
macro_rules! Depcrate_rc_test_objectimpl_103 {
() => {
// Module: crate::rc_test_object
// Provides: {"impl_103"}
// Dependencies: {}
impl ThreadTestData { # [doc = " Get the amount of method calls performed on the current thread."] pub (crate) fn current () -> Self { TEST_DATA . with (| data | data . borrow () . clone ()) } # [track_caller] # [allow (clippy :: missing_panics_doc)] # [allow (dead_code)] pub (crate) fn assert_current (& self) { let current = Self :: current () ; let mut expected = self . clone () ; if cfg ! (feature = "gnustep-1-7") { let retain_diff = expected . try_retain - current . try_retain ; expected . retain += retain_diff ; expected . try_retain -= retain_diff ; expected . autorelease = 0 ; } if current != expected { panic ! ("got differing amounts of calls:
   current: `{current:?}`,
  expected: `{expected:?}`") } } }
};
}
