// Generated macro for KEYLOGFILE_ENABLED (const)
macro_rules! Depcrate_settings_configKEYLOGFILE_ENABLED {
() => {
// Module: crate::settings::config
// Provides: {"KEYLOGFILE_ENABLED"}
// Dependencies: {}
# [doc = " Whether `--cfg capture_keylogs` was set at build time. We keep supporting"] # [doc = " the `capture_keylogs` feature for backward compatibility."] const KEYLOGFILE_ENABLED : bool = cfg ! (capture_keylogs) || cfg ! (feature = "capture_keylogs") ;
};
}
