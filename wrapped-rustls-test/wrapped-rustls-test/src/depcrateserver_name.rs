// Generated macro for server_name (function)
macro_rules! Depcrateserver_name {
() => {
// Module: crate
// Provides: {"server_name"}
// Dependencies: {}
pub fn server_name (name : & 'static str) -> ServerName < 'static > { name . try_into () . unwrap () }
};
}
