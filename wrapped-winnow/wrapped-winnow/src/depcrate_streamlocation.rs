// Generated macro for Location (trait)
macro_rules! Depcrate_streamLocation {
() => {
// Module: crate::stream
// Provides: {"Location"}
// Dependencies: {}
# [doc = " Current parse locations offset"] # [doc = ""] # [doc = " See [`LocatingSlice`] for adding location tracking to your [`Stream`]"] pub trait Location { # [doc = " Previous token's end offset"] fn previous_token_end (& self) -> usize ; # [doc = " Current token's start offset"] fn current_token_start (& self) -> usize ; }
};
}
