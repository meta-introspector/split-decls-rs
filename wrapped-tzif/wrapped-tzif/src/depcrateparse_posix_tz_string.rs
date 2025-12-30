// Generated macro for parse_posix_tz_string (function)
macro_rules! Depcrateparse_posix_tz_string {
() => {
// Module: crate
// Provides: {"parse_posix_tz_string"}
// Dependencies: {}
# [doc = " Parses a POSIX time-zone string from the given bytes."] pub fn parse_posix_tz_string (bytes : & [u8]) -> Result < PosixTzString , Error > { Ok (parse :: posix :: posix_tz_string () . parse (bytes) ? . 0) }
};
}
