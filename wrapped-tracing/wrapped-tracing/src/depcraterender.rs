// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
# [instrument (skip_all)] fn render (frame : & mut Frame , events : & [Event]) { trace ! (frame_count = frame . count () , event_count = events . len ()) ; let events = events . iter () . map (| e | format ! ("{e:?}")) . collect :: < Vec < _ > > () ; let paragraph = Paragraph :: new (events . join ("\n")) . block (Block :: bordered () . title ("Tracing example. Press 'q' to quit.")) ; frame . render_widget (paragraph , frame . area ()) ; }
};
}
