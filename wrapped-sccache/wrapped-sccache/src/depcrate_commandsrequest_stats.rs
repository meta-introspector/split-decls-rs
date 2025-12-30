// Generated macro for request_stats (function)
macro_rules! Depcrate_commandsrequest_stats {
() => {
// Module: crate::commands
// Provides: {"request_stats"}
// Dependencies: {}
# [doc = " Send a `GetStats` request to the server, and return the `ServerInfo` request if successful."] pub fn request_stats (mut conn : ServerConnection) -> Result < ServerInfo > { debug ! ("request_stats") ; let response = conn . request (Request :: GetStats) . context ("Failed to send data to or receive data from server. Mismatch of client/server versions?" ,) ? ; if let Response :: Stats (stats) = response { Ok (* stats) } else { bail ! ("Unexpected server response!") } }
};
}
