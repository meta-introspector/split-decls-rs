// Generated macro for macro_8 (macro)
macro_rules! Depcratemacro_8 {
() => {
// Module: crate
// Provides: {"macro_8"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "unknown-ci")] { mod unknown ; use crate :: unknown as sys ; # [cfg (test)] pub (crate) const MIN_USERS : usize = 0 ; } else if # [cfg (any (target_os = "macos" , target_os = "ios" , target_os = "linux" , target_os = "android" , target_os = "freebsd"))] { mod unix ; use crate :: unix :: sys as sys ; # [cfg (feature = "network")] mod network ; # [cfg (feature = "network")] use crate :: unix :: network_helper ; # [cfg (test)] pub (crate) const MIN_USERS : usize = 1 ; } else if # [cfg (windows)] { mod windows ; use crate :: windows as sys ; # [cfg (feature = "network")] mod network ; # [cfg (feature = "network")] use crate :: windows :: network_helper ; # [cfg (test)] pub (crate) const MIN_USERS : usize = 1 ; } else { mod unknown ; use crate :: unknown as sys ; # [cfg (test)] pub (crate) const MIN_USERS : usize = 0 ; } }
};
}
