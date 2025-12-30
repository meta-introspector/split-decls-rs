// Generated macro for backwards_iter (module)
macro_rules! Depcrate_aggregatebackwards_iter {
() => {
// Module: crate::aggregate
// Provides: {"backwards_iter"}
// Dependencies: {}
mod backwards_iter { pub trait BackwardsIterator { type Item ; fn next_back (& mut self) -> Option < Self :: Item > ; } pub struct Rev < I > (I) ; pub trait BackwardsIteratorExt : Sized { fn rev (self) -> Rev < Self > ; } impl < I : BackwardsIterator > BackwardsIteratorExt for I { fn rev (self) -> Rev < Self > { Rev (self) } } impl < I : BackwardsIterator > Iterator for Rev < I > { type Item = I :: Item ; fn next (& mut self) -> Option < I :: Item > { self . 0 . next_back () } } }
};
}
