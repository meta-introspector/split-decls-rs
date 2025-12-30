// Generated macro for EchConfigPayload (enum)
macro_rules! Depcrate_msgs_handshakeEchConfigPayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"EchConfigPayload"}
// Dependencies: {}
# [doc = " An encrypted client hello (ECH) config."] # [non_exhaustive] # [derive (Clone , Debug , PartialEq)] pub (crate) enum EchConfigPayload { # [doc = " A recognized V18 ECH configuration."] V18 (EchConfigContents) , # [doc = " An unknown version ECH configuration."] Unknown { version : EchVersion , contents : PayloadU16 , } , }
};
}
