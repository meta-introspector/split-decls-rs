// Generated macro for macro_568 (macro)
macro_rules! Depcrate_passwords_optionsmacro_568 {
() => {
// Module: crate::passwords_options
// Provides: {"macro_568"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The option flags used to configure the evaluation of a `SecAccessControl`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct AccessControlOptions : CFOptionFlags { # [doc = " Constraint to access an item with either biometry or passcode. "] const USER_PRESENCE = kSecAccessControlUserPresence ; # [cfg (feature = "OSX_10_13")] # [doc = " Constraint to access an item with Touch ID for any enrolled fingers, or Face ID. "] const BIOMETRY_ANY = kSecAccessControlBiometryAny ; # [cfg (feature = "OSX_10_13")] # [doc = " Constraint to access an item with Touch ID for currently enrolled fingers, or from Face ID with the currently enrolled user. "] const BIOMETRY_CURRENT_SET = kSecAccessControlBiometryCurrentSet ; # [doc = " Constraint to access an item with a passcode. "] const DEVICE_PASSCODE = kSecAccessControlDevicePasscode ; # [cfg (feature = "OSX_10_15")] # [doc = " Constraint to access an item with a watch. "] const WATCH = kSecAccessControlWatch ; # [doc = " Indicates that at least one constraint must be satisfied. "] const OR = kSecAccessControlOr ; # [doc = " Indicates that all constraints must be satisfied. "] const AND = kSecAccessControlAnd ; # [doc = " Enable a private key to be used in signing a block of data or verifying a signed block. "] const PRIVATE_KEY_USAGE = kSecAccessControlPrivateKeyUsage ; # [doc = " Option to use an application-provided password for data encryption key generation. "] const APPLICATION_PASSWORD = kSecAccessControlApplicationPassword ; } }
};
}
