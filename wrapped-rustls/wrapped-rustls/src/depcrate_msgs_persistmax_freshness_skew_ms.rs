// Generated macro for MAX_FRESHNESS_SKEW_MS (static)
macro_rules! Depcrate_msgs_persistMAX_FRESHNESS_SKEW_MS {
() => {
// Module: crate::msgs::persist
// Provides: {"MAX_FRESHNESS_SKEW_MS"}
// Dependencies: {}
# [doc = " This is the maximum allowed skew between server and client clocks, over"] # [doc = " the maximum ticket lifetime period.  This encompasses TCP retransmission"] # [doc = " times in case packet loss occurs when the client sends the ClientHello"] # [doc = " or receives the NewSessionTicket, _and_ actual clock skew over this period."] static MAX_FRESHNESS_SKEW_MS : u32 = 60 * 1000 ;
};
}
