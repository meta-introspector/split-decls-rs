// Generated macro for Side (enum)
macro_rules! Depcrate_common_stateSide {
() => {
// Module: crate::common_state
// Provides: {"Side"}
// Dependencies: {}
# [doc = " Side of the connection."] # [expect (clippy :: exhaustive_enums)] # [derive (Clone , Copy , Debug , PartialEq)] pub enum Side { # [doc = " A client initiates the connection."] Client , # [doc = " A server waits for a client to connect."] Server , }
};
}
