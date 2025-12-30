// Generated macro for IpNetworkFromStrError (enum)
macro_rules! Depcrate_common_networkIpNetworkFromStrError {
() => {
// Module: crate::common::network
// Provides: {"IpNetworkFromStrError"}
// Dependencies: {}
# [doc = " Error type returned from `MacAddr::from_str` implementation."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum IpNetworkFromStrError { # [doc = " Prefix is not an integer."] PrefixError (ParseIntError) , # [doc = " Failed to parse IP address."] AddrParseError (AddrParseError) , # [doc = " Input is not of format `[IP address]/[number]`."] InvalidAddrFormat , }
};
}
