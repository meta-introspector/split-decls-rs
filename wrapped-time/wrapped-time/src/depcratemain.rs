// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut log = std :: fs :: File :: create ("D:\\service.txt") . unwrap () ; Service :: new () . can_stop () . can_accept (SERVICE_ACCEPT_TIMECHANGE) . run (| _service , command | { writeln ! (log , "Command: {command:?}") . unwrap () ; if let Command :: Extended (command) = command { if command . control == SERVICE_CONTROL_TIMECHANGE { unsafe { let data = & * (command . data as * const SERVICE_TIMECHANGE_INFO) ; writeln ! (log , "{data:#?}") . unwrap () ; let old = convert (data . liOldTime) ; let new = convert (data . liNewTime) ; writeln ! (log , "{old:#?}\n{new:#?}") . unwrap () ; } } } }) . unwrap () ; }
};
}
