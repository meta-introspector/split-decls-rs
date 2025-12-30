// Generated macro for IteratorAsExactSizeIterator (struct)
macro_rules! Depcrate_iterator_as_exact_size_iteratorIteratorAsExactSizeIterator {
() => {
// Module: crate::iterator_as_exact_size_iterator
// Provides: {"IteratorAsExactSizeIterator"}
// Dependencies: {}
# [doc = " Wrap an iterator and implement `ExactSizeIterator`"] # [doc = " assuming the underlying iterator reports lower bound equal to upper bound."] # [doc = ""] # [doc = " It does not check the size is reported correctly (except in debug mode)."] pub (crate) struct IteratorAsExactSizeIterator < I > { iter : I , }
};
}
