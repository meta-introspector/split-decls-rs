// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { CoInitialize (None) . ok () ? } ; shell_execute_from_explorer ("https://github.com/microsoft/windows-rs" , "" , "" , "" , SW_SHOWNORMAL ,) }
};
}
