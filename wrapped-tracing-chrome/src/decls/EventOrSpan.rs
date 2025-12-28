macro_rules! EventOrSpan {
    () => {
        # [doc = " Represents either an [`Event`](tracing_core::Event) or [`SpanRef`](tracing_subscriber::registry::SpanRef)."] pub enum EventOrSpan < 'a , 'b , S > where S : Subscriber + for < 'span > LookupSpan < 'span > + Send + Sync , { Event (& 'a Event < 'b >) , Span (& 'a SpanRef < 'b , S >) , }
    };
}

EventOrSpan!()