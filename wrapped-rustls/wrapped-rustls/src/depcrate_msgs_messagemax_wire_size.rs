// Generated macro for MAX_WIRE_SIZE (const)
macro_rules! Depcrate_msgs_messageMAX_WIRE_SIZE {
() => {
// Module: crate::msgs::message
// Provides: {"MAX_WIRE_SIZE"}
// Dependencies: {}
# [doc = " Maximum on-the-wire message size."] # [cfg (feature = "std")] pub (crate) const MAX_WIRE_SIZE : usize = MAX_PAYLOAD as usize + HEADER_SIZE ;
};
}
