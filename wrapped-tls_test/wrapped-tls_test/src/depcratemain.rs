// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { thread_local ! { static S : RefCell < String > = RefCell :: default () ; } S . with (| x | * x . borrow_mut () = "pika pika" . to_string ()) ; S . with (| x | println ! ("{}" , x . borrow ())) ; }
};
}
