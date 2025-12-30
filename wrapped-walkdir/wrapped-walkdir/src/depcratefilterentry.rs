// Generated macro for FilterEntry (struct)
macro_rules! DepcrateFilterEntry {
() => {
// Module: crate
// Provides: {"FilterEntry"}
// Dependencies: {}
# [doc = " A recursive directory iterator that skips entries."] # [doc = ""] # [doc = " Values of this type are created by calling [`.filter_entry()`] on an"] # [doc = " `IntoIter`, which is formed by calling [`.into_iter()`] on a `WalkDir`."] # [doc = ""] # [doc = " Directories that fail the predicate `P` are skipped. Namely, they are"] # [doc = " never yielded and never descended into."] # [doc = ""] # [doc = " Entries that are skipped with the [`min_depth`] and [`max_depth`] options"] # [doc = " are not passed through this filter."] # [doc = ""] # [doc = " If opening a handle to a directory resulted in an error, then it is yielded"] # [doc = " and no corresponding call to the predicate is made."] # [doc = ""] # [doc = " Type parameter `I` refers to the underlying iterator and `P` refers to the"] # [doc = " predicate, which is usually `FnMut(&DirEntry) -> bool`."] # [doc = ""] # [doc = " [`.filter_entry()`]: struct.IntoIter.html#method.filter_entry"] # [doc = " [`.into_iter()`]: struct.WalkDir.html#into_iter.v"] # [doc = " [`min_depth`]: struct.WalkDir.html#method.min_depth"] # [doc = " [`max_depth`]: struct.WalkDir.html#method.max_depth"] # [derive (Debug)] pub struct FilterEntry < I , P > { it : I , predicate : P , }
};
}
