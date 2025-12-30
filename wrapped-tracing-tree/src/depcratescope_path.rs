// Generated macro for scope_path (function)
macro_rules! Depcratescope_path {
() => {
// Module: crate
// Provides: {"scope_path"}
// Dependencies: {}
fn scope_path < 'a , R : LookupSpan < 'a > > (span : & SpanRef < 'a , R >) -> ScopeFromRoot < 'a , R > { span . scope () . from_root () }
};
}
