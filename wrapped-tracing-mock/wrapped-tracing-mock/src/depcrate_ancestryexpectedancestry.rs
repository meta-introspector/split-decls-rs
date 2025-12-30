// Generated macro for ExpectedAncestry (enum)
macro_rules! Depcrate_ancestryExpectedAncestry {
() => {
// Module: crate::ancestry
// Provides: {"ExpectedAncestry"}
// Dependencies: {}
# [doc = " The ancestry of an event or span."] # [doc = ""] # [doc = " An event or span can have an explicitly assigned parent, or be an explicit root. Otherwise,"] # [doc = " an event or span may have a contextually assigned parent or in the final case will be a"] # [doc = " contextual root."] # [derive (Debug , Eq , PartialEq)] pub enum ExpectedAncestry { # [doc = " The event or span has an explicitly assigned parent (created with `parent: span_id`) span."] HasExplicitParent (ExpectedSpan) , # [doc = " The event or span is an explicitly defined root. It was created with `parent: None` and"] # [doc = " has no parent."] IsExplicitRoot , # [doc = " The event or span has a contextually assigned parent span. It has no explicitly assigned"] # [doc = " parent span, nor has it been explicitly defined as a root (it was created without the"] # [doc = " `parent:` directive). There was a span in context when this event or span was created."] HasContextualParent (ExpectedSpan) , # [doc = " The event or span is a contextual root. It has no explicitly assigned parent, nor has it"] # [doc = " been explicitly defined as a root (it was created without the `parent:` directive)."] # [doc = " Additionally, no span was in context when this event or span was created."] IsContextualRoot , }
};
}
