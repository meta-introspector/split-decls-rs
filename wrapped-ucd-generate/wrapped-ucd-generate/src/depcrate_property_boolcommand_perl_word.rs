// Generated macro for command_perl_word (function)
macro_rules! Depcrate_property_boolcommand_perl_word {
() => {
// Module: crate::property_bool
// Provides: {"command_perl_word"}
// Dependencies: {}
pub fn command_perl_word (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let props = parse_properties (& dir) ? ; let gencats = parse_general_categories (& dir) ? ; let mut perlword = BTreeSet :: new () ; perlword . extend (& props ["Alphabetic"]) ; perlword . extend (& props ["Join_Control"]) ; perlword . extend (& gencats ["Decimal_Number"]) ; perlword . extend (& gencats ["Nonspacing_Mark"]) ; perlword . extend (& gencats ["Enclosing_Mark"]) ; perlword . extend (& gencats ["Spacing_Mark"]) ; perlword . extend (& gencats ["Connector_Punctuation"]) ; let mut wtr = args . writer ("perl_word") ? ; wtr . ranges (args . name () , & perlword) ? ; Ok (()) }
};
}
