// Generated macro for EventOrSpan (enum)
macro_rules! DepcrateEventOrSpan {
() => {
// Module: crate
// Provides: {"EventOrSpan"}
// Dependencies: {}
# [doc = " Represents either an [`Event`](tracing_core::Event) or [`SpanRef`](tracing_subscriber::registry::SpanRef)."] pub enum EventOrSpan < 'a , 'b , S > where S : Subscriber + for < 'span > LookupSpan < 'span > + Send + Sync , { Event (& 'a Event < 'b >) , Span (& 'a SpanRef < 'b , S >) , }
};
}
