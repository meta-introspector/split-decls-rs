// Generated macro for IdentifyDistinct (struct)
macro_rules! Depcrate_algorithms_utilsIdentifyDistinct {
() => {
// Module: crate::algorithms::utils
// Provides: {"IdentifyDistinct"}
// Dependencies: {}
# [doc = " A utility struct to convert distinct items to unique integers."] # [doc = ""] # [doc = " This can be helpful on larger inputs to speed up the comparisons"] # [doc = " performed by doing a first pass where the data set gets reduced"] # [doc = " to (small) integers."] # [doc = ""] # [doc = " The idea is that instead of passing two sequences to a diffling algorithm"] # [doc = " you first pass it via [`IdentifyDistinct`]:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::capture_diff;"] # [doc = " use similar::algorithms::{Algorithm, IdentifyDistinct};"] # [doc = ""] # [doc = " let old = &[\"foo\", \"bar\", \"baz\"][..];"] # [doc = " let new = &[\"foo\", \"blah\", \"baz\"][..];"] # [doc = " let h = IdentifyDistinct::<u32>::new(old, 0..old.len(), new, 0..new.len());"] # [doc = " let ops = capture_diff("] # [doc = "     Algorithm::Myers,"] # [doc = "     h.old_lookup(),"] # [doc = "     h.old_range(),"] # [doc = "     h.new_lookup(),"] # [doc = "     h.new_range(),"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " The indexes are the same as with the passed source ranges."] pub struct IdentifyDistinct < Int > { old : OffsetLookup < Int > , new : OffsetLookup < Int > , }
};
}
