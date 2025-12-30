// Generated macro for Action (enum)
macro_rules! Depcrate_ci_cAction {
() => {
// Module: crate::ci::c
// Provides: {"Action"}
// Dependencies: {}
# [derive (Subcommand)] pub enum Action { # [doc = " Build image."] Build , Firecracker (super :: firecracker :: Firecracker) , Qemu (super :: qemu :: Qemu) , Uhyve (super :: uhyve :: Uhyve) , }
};
}
