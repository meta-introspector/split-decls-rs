// Generated macro for impl_1271 (impl)
macro_rules! Depcrate_ioimpl_1271 {
() => {
// Module: crate::io
// Provides: {"impl_1271"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : BufRead > Iterator for Split < B > { type Item = Result < Vec < u8 > > ; fn next (& mut self) -> Option < Result < Vec < u8 > > > { let mut buf = Vec :: new () ; match self . buf . read_until (self . delim , & mut buf) { Ok (0) => None , Ok (_n) => { if buf [buf . len () - 1] == self . delim { buf . pop () ; } Some (Ok (buf)) } Err (e) => Some (Err (e)) , } } }
};
}
