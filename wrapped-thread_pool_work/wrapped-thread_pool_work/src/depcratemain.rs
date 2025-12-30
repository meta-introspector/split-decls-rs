// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { let work = CreateThreadpoolWork (Some (callback) , None , None) ? ; for _ in 0 .. 10 { SubmitThreadpoolWork (work) ; } WaitForThreadpoolWorkCallbacks (work , false) ; } let counter = COUNTER . read () . unwrap () ; println ! ("counter: {}" , * counter) ; Ok (()) }
};
}
