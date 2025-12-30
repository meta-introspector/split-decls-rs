// Generated macro for id (function)
macro_rules! Depcrate_expectid {
() => {
// Module: crate::expect
// Provides: {"id"}
// Dependencies: {}
# [doc = " Returns a new, unset `ExpectedId`."] # [doc = ""] # [doc = " The `ExpectedId` needs to be attached to a [`NewSpan`] or an"] # [doc = " [`ExpectedSpan`] passed to [`MockSubscriber::new_span`] to"] # [doc = " ensure that it gets set. When the a clone of the same"] # [doc = " `ExpectedSpan` is attached to an [`ExpectedSpan`] and passed to"] # [doc = " any other method on [`MockSubscriber`] that accepts it, it will"] # [doc = " ensure that it is exactly the same span used across those"] # [doc = " distinct expectations."] # [doc = ""] # [doc = " For more details on how to use this struct, see the documentation"] # [doc = " on [`ExpectedSpan::with_id`]."] # [doc = ""] # [doc = " [`MockSubscriber`]: struct@crate::subscriber::MockSubscriber"] # [doc = " [`MockSubscriber::new_span`]: fn@crate::subscriber::MockSubscriber::new_span"] pub fn id () -> ExpectedId { ExpectedId :: new_unset () }
};
}
