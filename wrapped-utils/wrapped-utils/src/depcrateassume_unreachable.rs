// Generated macro for assume_unreachable (macro)
macro_rules! Depcrateassume_unreachable {
() => {
// Module: crate
// Provides: {"assume_unreachable"}
// Dependencies: {}
macro_rules ! assume_unreachable { () => { if cfg ! (debug_assertions) { panic ! () } else { core :: hint :: unreachable_unchecked () } } ; }
};
}
