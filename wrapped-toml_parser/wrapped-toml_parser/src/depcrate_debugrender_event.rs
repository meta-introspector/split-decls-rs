// Generated macro for render_event (function)
macro_rules! Depcrate_debugrender_event {
() => {
// Module: crate::debug
// Provides: {"render_event"}
// Dependencies: {}
fn render_event (span : impl Into < Option < Span > > , text : & str , style : anstyle :: Style) { # ! [allow (unexpected_cfgs)] let span = span . into () ; let depth = DEBUG_DEPTH . depth () . min (20) ; anstream :: eprintln ! ("{:depth$}{style}{text}: {span:?}{style:#}" , "") ; }
};
}
