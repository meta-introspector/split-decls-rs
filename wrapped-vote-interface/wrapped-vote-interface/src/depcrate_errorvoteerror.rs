// Generated macro for VoteError (enum)
macro_rules! Depcrate_errorVoteError {
() => {
// Module: crate::error
// Provides: {"VoteError"}
// Dependencies: {}
# [doc = " Reasons the vote might have had an error"] # [derive (Debug , Clone , PartialEq , Eq , FromPrimitive , ToPrimitive)] pub enum VoteError { VoteTooOld , SlotsMismatch , SlotHashMismatch , EmptySlots , TimestampTooOld , TooSoonToReauthorize , LockoutConflict , NewVoteStateLockoutMismatch , SlotsNotOrdered , ConfirmationsNotOrdered , ZeroConfirmations , ConfirmationTooLarge , RootRollBack , ConfirmationRollBack , SlotSmallerThanRoot , TooManyVotes , VotesTooOldAllFiltered , RootOnDifferentFork , ActiveVoteAccountClose , CommissionUpdateTooLate , AssertionFailed , }
};
}
