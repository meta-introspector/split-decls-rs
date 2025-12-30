// Generated macro for get_file_line (function)
macro_rules! Depcrate_unix_linux_componentget_file_line {
() => {
// Module: crate::unix::linux::component
// Provides: {"get_file_line"}
// Dependencies: {}
fn get_file_line (file : & Path , capacity : usize) -> Option < String > { let mut reader = String :: with_capacity (capacity) ; let mut f = File :: open (file) . ok () ? ; f . read_to_string (& mut reader) . ok () ? ; reader . truncate (reader . trim_end () . len ()) ; Some (reader) }
};
}
