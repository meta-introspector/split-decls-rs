// Generated macro for request_zero_stats (function)
macro_rules! Depcrate_commandsrequest_zero_stats {
() => {
// Module: crate::commands
// Provides: {"request_zero_stats"}
// Dependencies: {}
# [doc = " Send a `ZeroStats` request to the server, and return the `ServerInfo` request if successful."] pub fn request_zero_stats (mut conn : ServerConnection) -> Result < () > { debug ! ("request_stats") ; let response = conn . request (Request :: ZeroStats) . context ("failed to send zero statistics command to server or failed to receive response" ,) ? ; if let Response :: ZeroStats = response { Ok (()) } else { bail ! ("Unexpected server response!") } }
};
}
