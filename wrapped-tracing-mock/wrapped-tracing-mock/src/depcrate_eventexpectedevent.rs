// Generated macro for ExpectedEvent (struct)
macro_rules! Depcrate_eventExpectedEvent {
() => {
// Module: crate::event
// Provides: {"ExpectedEvent"}
// Dependencies: {}
# [doc = " An expected event."] # [doc = ""] # [doc = " For a detailed description and examples, see the documentation for"] # [doc = " the methods and the [`event`] module."] # [doc = ""] # [doc = " [`event`]: mod@crate::event"] # [derive (Default , Eq , PartialEq)] pub struct ExpectedEvent { pub (super) fields : Option < field :: ExpectedFields > , pub (super) ancestry : Option < ExpectedAncestry > , pub (super) in_spans : Option < Vec < span :: ExpectedSpan > > , pub (super) metadata : ExpectedMetadata , }
};
}
