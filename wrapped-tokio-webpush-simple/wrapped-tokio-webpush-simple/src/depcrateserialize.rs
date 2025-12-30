// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
fn serialize (message : ServerMessage) -> Result < Message > { let string = serde_json :: to_string (& message) . chain_err (| | "failed to serialize") ? ; Ok (Message :: Text (string)) }
};
}
