// Generated macro for parse_tzif_file (function)
macro_rules! Depcrateparse_tzif_file {
() => {
// Module: crate
// Provides: {"parse_tzif_file"}
// Dependencies: {}
# [doc = " Parses a `TZif` file at the provided `path`."] pub fn parse_tzif_file (path : & Path) -> Result < TzifData , Error > { let file = File :: open (path) ? ; let stream = stream :: buffered :: Stream :: new (stream :: position :: Stream :: new (stream :: read :: Stream :: new (file)) , 0 ,) ; Ok (parse :: tzif :: tzif () . parse (stream) ? . 0) }
};
}
