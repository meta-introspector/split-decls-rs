macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        # [doc = " A handle representing a span, with the capability to enter the span if it"] # [doc = " exists."] # [doc = ""] # [doc = " If the span was rejected by the current `Subscriber`'s filter, entering the"] # [doc = " span will silently do nothing. Thus, the handle can be used in the same"] # [doc = " manner regardless of whether or not the trace is currently being collected."] # [derive (Clone)] pub struct Span { # [doc = " A handle used to enter the span when it is not executing."] # [doc = ""] # [doc = " If this is `None`, then the span has either closed or was never enabled."] inner : Option < Inner > , # [doc = " Metadata describing the span."] # [doc = ""] # [doc = " This might be `Some` even if `inner` is `None`, in the case that the"] # [doc = " span is disabled but the metadata is needed for `log` support."] meta : Option < & 'static Metadata < 'static > > , }
    };
}

Span!();