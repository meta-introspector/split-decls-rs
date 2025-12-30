// Generated macro for raw_cargo (function)
macro_rules! Depcrate_cargoraw_cargo {
() => {
// Module: crate::cargo
// Provides: {"raw_cargo"}
// Dependencies: {}
fn raw_cargo () -> Command { match env :: var_os ("CARGO") { Some (cargo) => Command :: new (cargo) , None => Command :: new ("cargo") , } }
};
}
