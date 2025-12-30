// Generated macro for impl_131 (impl)
macro_rules! Depcrate_commonimpl_131 {
() => {
// Module: crate::common
// Provides: {"impl_131"}
// Dependencies: {}
impl FromStr for CodepointRange { type Err = Error ; fn from_str (s : & str) -> Result < CodepointRange , Error > { let re_parts = regex ! (r"^(?P<start>[A-Z0-9]+)\.\.(?P<end>[A-Z0-9]+)$") ; let caps = match re_parts . captures (s) { Some (caps) => caps , None => return err ! ("invalid codepoint range: '{}'" , s) , } ; let start = caps ["start"] . parse () . or_else (| err | { err ! ("failed to parse '{}' as a codepoint range: {}" , s , err) }) ? ; let end = caps ["end"] . parse () . or_else (| err | { err ! ("failed to parse '{}' as a codepoint range: {}" , s , err) }) ? ; Ok (CodepointRange { start , end }) } }
};
}
