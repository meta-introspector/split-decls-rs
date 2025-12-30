// Generated macro for features (function)
macro_rules! Depcrate_cargofeatures {
() => {
// Module: crate::cargo
// Provides: {"features"}
// Dependencies: {}
fn features (project : & Project) -> Vec < String > { match & project . features { Some (features) => vec ! ["--no-default-features" . to_owned () , "--features" . to_owned () , features . join (",") ,] , None => vec ! [] , } }
};
}
