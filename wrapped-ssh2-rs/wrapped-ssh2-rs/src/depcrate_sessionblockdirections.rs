// Generated macro for BlockDirections (enum)
macro_rules! Depcrate_sessionBlockDirections {
() => {
// Module: crate::session
// Provides: {"BlockDirections"}
// Dependencies: {}
# [doc = " The io direction an application has to wait for in order not to block."] # [derive (Debug , PartialEq)] pub enum BlockDirections { # [doc = " No direction blocked."] None , # [doc = " Inbound direction blocked."] Inbound , # [doc = " Outbound direction blockd."] Outbound , # [doc = " Inbound and Outbound direction blocked."] Both , }
};
}
