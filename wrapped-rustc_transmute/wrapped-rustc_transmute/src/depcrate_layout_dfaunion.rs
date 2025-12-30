// Generated macro for union (function)
macro_rules! Depcrate_layout_dfaunion {
() => {
// Module: crate::layout::dfa
// Provides: {"union"}
// Dependencies: {}
# [doc = " Merges two sorted sequences into one sorted sequence."] pub (crate) fn union < S : Copy , X : Iterator < Item = (Byte , S) > , Y : Iterator < Item = (Byte , S) > > (xs : X , ys : Y ,) -> UnionIter < X , Y > { UnionIter { xs : xs . peekable () , ys : ys . peekable () } }
};
}
