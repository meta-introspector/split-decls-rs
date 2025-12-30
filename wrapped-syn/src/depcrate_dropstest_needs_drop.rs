// Generated macro for test_needs_drop (function)
macro_rules! Depcrate_dropstest_needs_drop {
() => {
// Module: crate::drops
// Provides: {"test_needs_drop"}
// Dependencies: {}
# [test] fn test_needs_drop () { use std :: mem :: needs_drop ; struct NeedsDrop ; impl Drop for NeedsDrop { fn drop (& mut self) { } } assert ! (needs_drop ::< NeedsDrop > ()) ; assert ! (! needs_drop ::< iter :: Empty < NeedsDrop >> ()) ; assert ! (! needs_drop ::< slice :: Iter < NeedsDrop >> ()) ; assert ! (! needs_drop ::< slice :: IterMut < NeedsDrop >> ()) ; assert ! (! needs_drop ::< option :: IntoIter <& NeedsDrop >> ()) ; assert ! (! needs_drop ::< option :: IntoIter <& mut NeedsDrop >> ()) ; }
};
}
