// Generated macro for command (function)
macro_rules! Depcrate_jamo_short_namecommand {
() => {
// Module: crate::jamo_short_name
// Provides: {"command"}
// Dependencies: {}
pub fn command (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let map = jamo_map (& Path :: new (dir)) ? ; let mut wtr = args . writer ("jamo_short_name") ? ; wtr . codepoint_to_string (args . name () , & map) ? ; Ok (()) }
};
}
