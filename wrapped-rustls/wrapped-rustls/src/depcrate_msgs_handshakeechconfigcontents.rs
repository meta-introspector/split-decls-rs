// Generated macro for EchConfigContents (struct)
macro_rules! Depcrate_msgs_handshakeEchConfigContents {
() => {
// Module: crate::msgs::handshake
// Provides: {"EchConfigContents"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub (crate) struct EchConfigContents { pub key_config : HpkeKeyConfig , pub maximum_name_length : u8 , pub public_name : DnsName < 'static > , pub extensions : Vec < EchConfigExtension > , }
};
}
