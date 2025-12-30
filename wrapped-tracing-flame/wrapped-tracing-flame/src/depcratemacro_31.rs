// Generated macro for macro_31 (macro)
macro_rules! Depcratemacro_31 {
() => {
// Module: crate
// Provides: {"macro_31"}
// Dependencies: {}
thread_local ! { static LAST_EVENT : Cell < Instant > = Cell :: new (* START) ; static THREAD_NAME : String = { let thread = std :: thread :: current () ; let mut thread_name = format ! ("{:?}" , thread . id ()) ; if let Some (name) = thread . name () { thread_name += "-" ; thread_name += name ; } thread_name } ; }
};
}
