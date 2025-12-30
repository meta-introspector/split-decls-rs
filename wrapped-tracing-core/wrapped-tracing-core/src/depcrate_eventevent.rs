// Generated macro for Event (struct)
macro_rules! Depcrate_eventEvent {
() => {
// Module: crate::event
// Provides: {"Event"}
// Dependencies: {}
# [doc = " `Event`s represent single points in time where something occurred during the"] # [doc = " execution of a program."] # [doc = ""] # [doc = " An `Event` can be compared to a log record in unstructured logging, but with"] # [doc = " two key differences:"] # [doc = " - `Event`s exist _within the context of a [span]_. Unlike log lines, they"] # [doc = "   may be located within the trace tree, allowing visibility into the"] # [doc = "   _temporal_ context in which the event occurred, as well as the source"] # [doc = "   code location."] # [doc = " - Like spans, `Event`s have structured key-value data known as _[fields]_,"] # [doc = "   which may include textual message. In general, a majority of the data"] # [doc = "   associated with an event should be in the event's fields rather than in"] # [doc = "   the textual message, as the fields are more structured."] # [doc = ""] # [doc = " [span]: super::span"] # [doc = " [fields]: super::field"] # [derive (Debug)] pub struct Event < 'a > { fields : & 'a field :: ValueSet < 'a > , metadata : & 'static Metadata < 'static > , parent : Parent , }
};
}
