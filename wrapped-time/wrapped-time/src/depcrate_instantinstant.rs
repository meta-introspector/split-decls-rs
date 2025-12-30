// Generated macro for Instant (struct)
macro_rules! Depcrate_instantInstant {
() => {
// Module: crate::instant
// Provides: {"Instant"}
// Dependencies: {}
# [doc = " A measurement of a monotonically non-decreasing clock. Opaque and useful only with [`Duration`]."] # [doc = ""] # [doc = " Instants are always guaranteed to be no less than any previously measured instant when created,"] # [doc = " and are often useful for tasks such as measuring benchmarks or timing how long an operation"] # [doc = " takes."] # [doc = ""] # [doc = " Note, however, that instants are not guaranteed to be **steady**. In other words, each tick of"] # [doc = " the underlying clock may not be the same length (e.g. some seconds may be longer than others)."] # [doc = " An instant may jump forwards or experience time dilation (slow down or speed up), but it will"] # [doc = " never go backwards."] # [doc = ""] # [doc = " Instants are opaque types that can only be compared to one another. There is no method to get"] # [doc = " \"the number of seconds\" from an instant. Instead, it only allows measuring the duration between"] # [doc = " two instants (or comparing two instants)."] # [doc = ""] # [doc = " This implementation allows for operations with signed [`Duration`]s, but is otherwise identical"] # [doc = " to [`std::time::Instant`]."] # [doc (hidden)] # [deprecated (since = "0.3.35" , note = "import `std::time::Instant` and `time::ext::InstantExt` instead")] # [repr (transparent)] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Instant (pub StdInstant) ;
};
}
