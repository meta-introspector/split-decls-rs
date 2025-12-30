// Generated macro for from_linger (function)
macro_rules! Depcrate_socketfrom_linger {
() => {
// Module: crate::socket
// Provides: {"from_linger"}
// Dependencies: {}
const fn from_linger (linger : sys :: linger) -> Option < Duration > { if linger . l_onoff == 0 { None } else { Some (Duration :: from_secs (linger . l_linger as u64)) } }
};
}
