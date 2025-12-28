macro_rules! SpanRange {
    () => {
        struct SpanRange { start : Span , end : Span , }
    };
}

SpanRange!()