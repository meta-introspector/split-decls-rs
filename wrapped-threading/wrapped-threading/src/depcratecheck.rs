// Generated macro for check (function)
macro_rules! Depcratecheck {
() => {
// Module: crate
// Provides: {"check"}
// Dependencies: {}
fn check < D : Default + PartialEq > (result : D) -> D { if result == D :: default () { panic ! ("allocation failed") ; } result }
};
}
