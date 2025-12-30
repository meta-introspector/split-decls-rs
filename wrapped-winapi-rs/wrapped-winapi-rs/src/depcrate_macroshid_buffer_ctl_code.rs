// Generated macro for HID_BUFFER_CTL_CODE (macro)
macro_rules! Depcrate_macrosHID_BUFFER_CTL_CODE {
() => {
// Module: crate::macros
// Provides: {"HID_BUFFER_CTL_CODE"}
// Dependencies: {}
macro_rules ! HID_BUFFER_CTL_CODE { ($ id : expr) => { CTL_CODE ! (FILE_DEVICE_KEYBOARD , $ id , METHOD_BUFFERED , FILE_ANY_ACCESS) } }
};
}
