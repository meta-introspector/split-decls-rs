// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < S , N > FormatEvent < S , N > for BacktraceFormatter where S : Subscriber + for < 'a > tracing_subscriber :: registry :: LookupSpan < 'a > , N : for < 'a > FormatFields < 'a > + 'static , { fn format_event (& self , _ctx : & FmtContext < '_ , S , N > , mut writer : format :: Writer < '_ > , event : & Event < '_ > ,) -> fmt :: Result { let target = event . metadata () . target () ; if ! target . contains (& self . backtrace_target) { return Ok (()) ; } let backtrace = std :: backtrace :: Backtrace :: force_capture () ; writeln ! (writer , "stack backtrace: \n{backtrace:?}") } }
};
}
