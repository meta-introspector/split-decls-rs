// Generated macro for eq (function)
macro_rules! Depcrateeq {
() => {
// Module: crate
// Provides: {"eq"}
// Dependencies: {}
fn eq (s1 : & [u8] , s2 : & [u8]) -> bool { if s1 . len () != s2 . len () { return false ; } s1 . iter () . zip (s2) . all (| (c1 , c2) | * c1 == * c2) }
};
}
