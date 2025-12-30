// Generated macro for DEFINE_BLUETOOTH_UUID128 (macro)
macro_rules! Depcrate_macrosDEFINE_BLUETOOTH_UUID128 {
() => {
// Module: crate::macros
// Provides: {"DEFINE_BLUETOOTH_UUID128"}
// Dependencies: {}
macro_rules ! DEFINE_BLUETOOTH_UUID128 { ($ name : ident , $ shortId : expr) => { DEFINE_GUID ! { $ name , $ shortId as u32 , 0x0000 , 0x1000 , 0x80 , 0x00 , 0x00 , 0x80 , 0x5F , 0x9B , 0x34 , 0xFB } } }
};
}
