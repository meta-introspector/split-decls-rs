// Generated macro for impl_173 (impl)
macro_rules! Depcrateimpl_173 {
() => {
// Module: crate
// Provides: {"impl_173"}
// Dependencies: {}
# [doc = " Parse a string as an URL, without a base URL or encoding override."] impl str :: FromStr for Url { type Err = ParseError ; # [inline] fn from_str (input : & str) -> Result < Self , crate :: ParseError > { Self :: parse (input) } }
};
}
