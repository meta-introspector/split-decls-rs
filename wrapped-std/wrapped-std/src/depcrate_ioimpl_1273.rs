// Generated macro for impl_1273 (impl)
macro_rules! Depcrate_ioimpl_1273 {
() => {
// Module: crate::io
// Provides: {"impl_1273"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : BufRead > Iterator for Lines < B > { type Item = Result < String > ; fn next (& mut self) -> Option < Result < String > > { let mut buf = String :: new () ; match self . buf . read_line (& mut buf) { Ok (0) => None , Ok (_n) => { if buf . ends_with ('\n') { buf . pop () ; if buf . ends_with ('\r') { buf . pop () ; } } Some (Ok (buf)) } Err (e) => Some (Err (e)) , } } }
};
}
