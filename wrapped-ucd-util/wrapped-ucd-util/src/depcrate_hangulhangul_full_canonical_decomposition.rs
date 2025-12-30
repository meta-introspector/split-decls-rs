// Generated macro for hangul_full_canonical_decomposition (function)
macro_rules! Depcrate_hangulhangul_full_canonical_decomposition {
() => {
// Module: crate::hangul
// Provides: {"hangul_full_canonical_decomposition"}
// Dependencies: {}
# [doc = " Return the full canonical decomposition of the given precomposed Hangul"] # [doc = " codepoint."] # [doc = ""] # [doc = " If the decomposition does not have any trailing consonant, then the third"] # [doc = " part of the tuple returned is `None`."] # [doc = ""] # [doc = " If the given codepoint does not correspond to a precomposed Hangul"] # [doc = " codepoint in the inclusive range `AC00..D7A3`, then this returns `None`."] # [doc = ""] # [doc = " This implements the algorithms described in Unicode 3.12 and Unicode 4.8."] pub fn hangul_full_canonical_decomposition (cp : u32 ,) -> Option < (u32 , u32 , Option < u32 >) > { if ! (0xAC00 <= cp && cp <= 0xD7A3) { return None ; } let s_index = cp - S_BASE ; let l_index = s_index / N_COUNT ; let v_index = (s_index % N_COUNT) / T_COUNT ; let t_index = s_index % T_COUNT ; let l_part = L_BASE + l_index ; let v_part = V_BASE + v_index ; let t_part = if t_index == 0 { None } else { Some (T_BASE + t_index) } ; Some ((l_part , v_part , t_part)) }
};
}
