// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let temperatures : Vec < u8 > = (0 .. 24) . map (| _ | rng () . random_range (50 .. 90)) . collect () ; ratatui :: run (| terminal | { loop { terminal . draw (| frame | render (frame , & temperatures)) ? ; if event :: read () ? . is_key_press () { break Ok (()) ; } } }) }
};
}
