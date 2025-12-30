// Generated macro for tidy_error (function)
macro_rules! Depcratetidy_error {
() => {
// Module: crate
// Provides: {"tidy_error"}
// Dependencies: {}
fn tidy_error (args : & str) -> std :: io :: Result < () > { use std :: io :: Write ; use termcolor :: { Color , ColorChoice , ColorSpec , StandardStream } ; let mut stderr = StandardStream :: stdout (ColorChoice :: Auto) ; stderr . set_color (ColorSpec :: new () . set_fg (Some (Color :: Red))) ? ; write ! (& mut stderr , "tidy error") ? ; stderr . set_color (& ColorSpec :: new ()) ? ; writeln ! (& mut stderr , ": {args}") ? ; Ok (()) }
};
}
