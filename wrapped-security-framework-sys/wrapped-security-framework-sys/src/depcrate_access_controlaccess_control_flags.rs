// Generated macro for access_control_flags (module)
macro_rules! Depcrate_access_controlaccess_control_flags {
() => {
// Module: crate::access_control
// Provides: {"access_control_flags"}
// Dependencies: {}
mod access_control_flags { use super :: CFOptionFlags ; pub const kSecAccessControlUserPresence : CFOptionFlags = 1 << 0 ; # [cfg (feature = "OSX_10_13")] pub const kSecAccessControlBiometryAny : CFOptionFlags = 1 << 1 ; # [cfg (feature = "OSX_10_13")] pub const kSecAccessControlBiometryCurrentSet : CFOptionFlags = 1 << 3 ; pub const kSecAccessControlDevicePasscode : CFOptionFlags = 1 << 4 ; # [cfg (feature = "OSX_10_15")] pub const kSecAccessControlWatch : CFOptionFlags = 1 << 5 ; pub const kSecAccessControlOr : CFOptionFlags = 1 << 14 ; pub const kSecAccessControlAnd : CFOptionFlags = 1 << 15 ; pub const kSecAccessControlPrivateKeyUsage : CFOptionFlags = 1 << 30 ; pub const kSecAccessControlApplicationPassword : CFOptionFlags = 1 << 31 ; }
};
}
