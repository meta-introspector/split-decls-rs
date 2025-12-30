// Generated macro for MacAddrFromStrError (enum)
macro_rules! Depcrate_common_networkMacAddrFromStrError {
() => {
// Module: crate::common::network
// Provides: {"MacAddrFromStrError"}
// Dependencies: {}
# [doc = " Error type returned from `MacAddr::from_str` implementation."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum MacAddrFromStrError { # [doc = " A number is not in hexadecimal format."] IntError (ParseIntError) , # [doc = " Input is not of format `{02X}:{02X}:{02X}:{02X}:{02X}:{02X}`."] InvalidAddrFormat , }
};
}
