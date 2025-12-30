// Generated macro for constantize (function)
macro_rules! Depcrateconstantize {
() => {
// Module: crate
// Provides: {"constantize"}
// Dependencies: {}
fn constantize (s : & str) -> String { let parts = s . split ("-") . map (| s | { s . chars () . enumerate () . map (| (n , c) | { if n == 0 { c . to_uppercase () . to_string () } else { c . to_string () } }) }) ; let mut res = String :: new () ; for part in parts { res . extend (part) ; } res }
};
}
