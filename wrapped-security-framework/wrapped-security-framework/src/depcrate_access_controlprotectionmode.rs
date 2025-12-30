// Generated macro for ProtectionMode (enum)
macro_rules! Depcrate_access_controlProtectionMode {
() => {
// Module: crate::access_control
// Provides: {"ProtectionMode"}
// Dependencies: {}
# [doc = " Specify when an item is available."] pub enum ProtectionMode { # [doc = " The data in the keychain can only be accessed when the device is"] # [doc = " unlocked. Only available if a passcode is set on the device."] AccessibleWhenPasscodeSetThisDeviceOnly , # [doc = "The data in the keychain item can be accessed only while the device is"] # [doc = " unlocked by the user."] AccessibleWhenUnlockedThisDeviceOnly , # [doc = " The data in the keychain item can be accessed only while the device is"] # [doc = " unlocked by the user."] AccessibleWhenUnlocked , # [doc = " The data in the keychain item cannot be accessed after a restart until"] # [doc = " the device has been unlocked once by the user."] AccessibleAfterFirstUnlockThisDeviceOnly , # [doc = " The data in the keychain item cannot be accessed after a restart until"] # [doc = " the device has been unlocked once by the user."] AccessibleAfterFirstUnlock , }
};
}
