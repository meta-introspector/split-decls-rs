// Generated macro for generate_asserts (function)
macro_rules! Depcrategenerate_asserts {
() => {
// Module: crate
// Provides: {"generate_asserts"}
// Dependencies: {}
fn generate_asserts (s : & mut String , prop : & str , points : & [u32] , truthy : bool ,) -> Result < () , fmt :: Error > { let truthy = if truthy { "" } else { "!" } ; for range in ranges_from_set (points) { let start = char :: from_u32 (range . start) . unwrap () ; let end = char :: from_u32 (range . end - 1) . unwrap () ; match range . len () { 1 => writeln ! (s , "        assert!({truthy}unicode_data::{prop}::lookup({start:?}));") ? , _ => { writeln ! (s , "        for c in {start:?}..={end:?} {{") ? ; writeln ! (s , "            assert!({truthy}unicode_data::{prop}::lookup(c));") ? ; writeln ! (s , "        }}") ? ; } } } Ok (()) }
};
}
