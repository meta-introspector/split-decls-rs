// Generated macro for HID_CTL_CODE (macro)
macro_rules! Depcrate_macrosHID_CTL_CODE {
() => {
// Module: crate::macros
// Provides: {"HID_CTL_CODE"}
// Dependencies: {}
macro_rules ! HID_CTL_CODE { ($ id : expr) => { CTL_CODE ! (FILE_DEVICE_KEYBOARD , $ id , METHOD_NEITHER , FILE_ANY_ACCESS) } }
};
}
