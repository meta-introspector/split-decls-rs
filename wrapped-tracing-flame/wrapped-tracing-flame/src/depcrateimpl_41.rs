// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < S , W > FlameLayer < S , W > where S : Subscriber + for < 'span > LookupSpan < 'span > , W : Write + 'static , { fn time_since_last_event (& self) -> Duration { let now = Instant :: now () ; let prev = LAST_EVENT . with (| e | { let prev = e . get () ; e . set (now) ; prev }) ; now - prev } }
};
}
