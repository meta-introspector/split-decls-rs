// Generated macro for tests (module)
macro_rules! Depcrate_backtracetests {
() => {
// Module: crate::backtrace
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: ErrorLayer ; use tracing :: subscriber :: with_default ; use tracing :: { span , Level } ; use tracing_subscriber :: { prelude :: * , registry :: Registry } ; # [test] fn capture_supported () { let subscriber = Registry :: default () . with (ErrorLayer :: default ()) ; with_default (subscriber , | | { let span = span ! (Level :: ERROR , "test span") ; let _guard = span . enter () ; let span_trace = SpanTrace :: capture () ; dbg ! (& span_trace) ; assert_eq ! (SpanTraceStatus :: CAPTURED , span_trace . status ()) }) ; } # [test] fn capture_empty () { let subscriber = Registry :: default () . with (ErrorLayer :: default ()) ; with_default (subscriber , | | { let span_trace = SpanTrace :: capture () ; dbg ! (& span_trace) ; assert_eq ! (SpanTraceStatus :: EMPTY , span_trace . status ()) }) ; } # [test] fn capture_unsupported () { let subscriber = Registry :: default () ; with_default (subscriber , | | { let span = span ! (Level :: ERROR , "test span") ; let _guard = span . enter () ; let span_trace = SpanTrace :: capture () ; dbg ! (& span_trace) ; assert_eq ! (SpanTraceStatus :: UNSUPPORTED , span_trace . status ()) }) ; } }
};
}
