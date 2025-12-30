// Generated macro for Hooks (struct)
macro_rules! Depcrate_settings_hooksHooks {
() => {
// Module: crate::settings::hooks
// Provides: {"Hooks"}
// Dependencies: {}
# [doc = " Hook configuration for use in the QUIC connection lifecycle."] # [doc = ""] # [doc = " Use these to manage the connection outside of what is possible with an"] # [doc = " [`ApplicationOverQuic`](crate::ApplicationOverQuic)."] # [derive (Default , Clone)] pub struct Hooks { pub connection_hook : Option < Arc < dyn ConnectionHook + Send + Sync + 'static > > , }
};
}
