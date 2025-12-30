// Generated macro for MSC_SET_BREAK_LENGTH (macro)
macro_rules! Depcrate_um_ws2bthMSC_SET_BREAK_LENGTH {
() => {
// Module: crate::um::ws2bth
// Provides: {"MSC_SET_BREAK_LENGTH"}
// Dependencies: {}
macro_rules ! MSC_SET_BREAK_LENGTH { ($ b : expr , $ l : expr) => { ($ b & 0x3) | (($ l & 0xf) << 4) } ; }
};
}
