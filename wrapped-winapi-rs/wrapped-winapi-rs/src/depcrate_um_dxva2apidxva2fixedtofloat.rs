// Generated macro for DXVA2FixedToFloat (function)
macro_rules! Depcrate_um_dxva2apiDXVA2FixedToFloat {
() => {
// Module: crate::um::dxva2api
// Provides: {"DXVA2FixedToFloat"}
// Dependencies: {}
# [inline] pub fn DXVA2FixedToFloat (_fixed_ : DXVA2_Fixed32) -> c_float { unsafe { _fixed_ . s () . Value as FLOAT + _fixed_ . s () . Fraction as FLOAT / 0x10000 as FLOAT } }
};
}
