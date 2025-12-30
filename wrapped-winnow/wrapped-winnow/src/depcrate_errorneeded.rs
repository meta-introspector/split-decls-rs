// Generated macro for Needed (enum)
macro_rules! Depcrate_errorNeeded {
() => {
// Module: crate::error
// Provides: {"Needed"}
// Dependencies: {}
# [doc = " Contains information on needed data if a parser returned `Incomplete`"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Note:** This is only possible for `Stream` that are [partial][`crate::stream::StreamIsPartial`],"] # [doc = " like [`Partial`][crate::Partial]."] # [doc = ""] # [doc = " </div>"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Needed { # [doc = " Needs more data, but we do not know how much"] Unknown , # [doc = " Contains a lower bound on the buffer offset needed to finish parsing"] # [doc = ""] # [doc = " For byte/`&str` streams, this translates to bytes"] Size (NonZeroUsize) , }
};
}
