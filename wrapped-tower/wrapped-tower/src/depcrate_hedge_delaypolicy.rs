// Generated macro for Policy (trait)
macro_rules! Depcrate_hedge_delayPolicy {
() => {
// Module: crate::hedge::delay
// Provides: {"Policy"}
// Dependencies: {}
# [doc = " A policy which specifies how long each request should be delayed for."] pub trait Policy < Request > { fn delay (& self , req : & Request) -> Duration ; }
};
}
