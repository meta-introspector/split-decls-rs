// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let _guard = init_tracing () ? ; info ! ("Starting tracing example") ; let mut terminal = ratatui :: init () ; let mut events = vec ! [] ; while ! should_exit (& events) { handle_events (& mut events) ? ; terminal . draw (| frame | render (frame , & events)) ? ; } ratatui :: restore () ; info ! ("Exiting tracing example") ; println ! ("See the tracing.log file for the logs") ; Ok (()) }
};
}
