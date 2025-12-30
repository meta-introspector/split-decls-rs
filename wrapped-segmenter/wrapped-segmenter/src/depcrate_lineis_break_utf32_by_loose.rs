// Generated macro for is_break_utf32_by_loose (function)
macro_rules! Depcrate_lineis_break_utf32_by_loose {
() => {
// Module: crate::line
// Provides: {"is_break_utf32_by_loose"}
// Dependencies: {}
# [inline] fn is_break_utf32_by_loose (right_codepoint : u32 , left_prop : u8 , right_prop : u8 , ja_zh : bool ,) -> Option < bool > { if right_prop == BA { if left_prop == ID && (right_codepoint == 0x2010 || right_codepoint == 0x2013) { return Some (true) ; } } else if right_prop == NS { if right_codepoint == 0x301C || right_codepoint == 0x30A0 { return Some (ja_zh) ; } if right_codepoint == 0x3005 || right_codepoint == 0x303B || right_codepoint == 0x309D || right_codepoint == 0x309E || right_codepoint == 0x30FD || right_codepoint == 0x30FE { return Some (true) ; } if right_codepoint == 0x30FB || right_codepoint == 0xFF1A || right_codepoint == 0xFF1B || right_codepoint == 0xFF65 || right_codepoint == 0x203C || (0x2047 ..= 0x2049) . contains (& right_codepoint) { return Some (ja_zh) ; } } else if right_prop == IN { return Some (true) ; } else if right_prop == EX { if right_codepoint == 0xFF01 || right_codepoint == 0xFF1F { return Some (ja_zh) ; } } if right_prop == PO_EAW { return Some (ja_zh) ; } if left_prop == PR_EAW { return Some (ja_zh) ; } None }
};
}
