// Generated macro for impl_82 (impl)
macro_rules! Depcrate_iterimpl_82 {
() => {
// Module: crate::iter
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , T , C : cfg :: Config > Iterator for UniqueIter < 'a , T , C > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { test_println ! ("UniqueIter::next") ; loop { test_println ! ("-> try next slot") ; if let Some (item) = self . slots . as_mut () . and_then (| slots | slots . next ()) { test_println ! ("-> found an item!") ; return Some (item) ; } test_println ! ("-> try next page") ; if let Some (page) = self . pages . next () { test_println ! ("-> found another page") ; self . slots = page . iter () ; continue ; } test_println ! ("-> try next shard") ; if let Some (shard) = self . shards . next () { test_println ! ("-> found another shard") ; self . pages = shard . iter () ; } else { test_println ! ("-> all done!") ; return None ; } } } }
};
}
