// Generated macro for decompose_hangul (function)
macro_rules! Depcrate_normalizedecompose_hangul {
() => {
// Module: crate::normalize
// Provides: {"decompose_hangul"}
// Dependencies: {}
# [allow (unsafe_code , unused_unsafe)] # [inline (always)] unsafe fn decompose_hangul < F > (s : char , mut emit_char : F) where F : FnMut (char) , { let s_index = s as u32 - S_BASE ; let l_index = s_index / N_COUNT ; unsafe { emit_char (char :: from_u32_unchecked (L_BASE + l_index)) ; let v_index = (s_index % N_COUNT) / T_COUNT ; emit_char (char :: from_u32_unchecked (V_BASE + v_index)) ; let t_index = s_index % T_COUNT ; if t_index > 0 { emit_char (char :: from_u32_unchecked (T_BASE + t_index)) ; } } }
};
}
