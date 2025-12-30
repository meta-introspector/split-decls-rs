// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
fn parse (message : Message) -> Result < ClientMessage > { match message { Message :: Text (s) => { println ! ("parse {}" , s) ; serde_json :: from_str (& s) . chain_err (| | "invalid json text") } Message :: Binary (slice) => { serde_json :: from_slice (& slice) . chain_err (| | "invalid json bytes") } } }
};
}
