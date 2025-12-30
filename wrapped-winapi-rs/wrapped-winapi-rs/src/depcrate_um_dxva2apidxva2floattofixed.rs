// Generated macro for DXVA2FloatToFixed (function)
macro_rules! Depcrate_um_dxva2apiDXVA2FloatToFixed {
() => {
// Module: crate::um::dxva2api
// Provides: {"DXVA2FloatToFixed"}
// Dependencies: {}
# [inline] pub fn DXVA2FloatToFixed (_float_ : c_float) -> DXVA2_Fixed32 { unsafe { let mut _fixed_ : DXVA2_Fixed32 = :: core :: mem :: uninitialized () ; _fixed_ . s_mut () . Fraction = LOWORD ((_float_ * 0x10000 as c_float) as DWORD) ; _fixed_ . s_mut () . Value = HIWORD ((_float_ * 0x10000 as c_float) as DWORD) as SHORT ; _fixed_ } }
};
}
