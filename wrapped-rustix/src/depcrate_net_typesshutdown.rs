// Generated macro for Shutdown (enum)
macro_rules! Depcrate_net_typesShutdown {
() => {
// Module: crate::net::types
// Provides: {"Shutdown"}
// Dependencies: {}
# [doc = " `SHUT_*` constants for use with [`shutdown`]."] # [doc = ""] # [doc = " [`shutdown`]: crate::net::shutdown"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u32)] pub enum Shutdown { # [doc = " `SHUT_RD`—Disable further read operations."] Read = c :: SHUT_RD as _ , # [doc = " `SHUT_WR`—Disable further write operations."] Write = c :: SHUT_WR as _ , # [doc = " `SHUT_RDWR`—Disable further read and write operations."] Both = c :: SHUT_RDWR as _ , }
};
}
