// Generated macro for default_port (function)
macro_rules! Depcrate_parserdefault_port {
() => {
// Module: crate::parser
// Provides: {"default_port"}
// Dependencies: {}
pub fn default_port (scheme : & str) -> Option < u16 > { match scheme { "http" | "ws" => Some (80) , "https" | "wss" => Some (443) , "ftp" => Some (21) , _ => None , } }
};
}
