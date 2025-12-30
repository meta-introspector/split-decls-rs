// Generated macro for w (macro)
macro_rules! Depcrate_literalsw {
() => {
// Module: crate::literals
// Provides: {"w"}
// Dependencies: {}
# [doc = " A literal UTF-16 wide string with a trailing null terminator."] # [macro_export] macro_rules ! w { ($ s : literal) => { { const INPUT : & [u8] = $ s . as_bytes () ; const OUTPUT_LEN : usize = $ crate :: utf16_len (INPUT) + 1 ; const OUTPUT : & [u16 ; OUTPUT_LEN] = { let mut buffer = [0 ; OUTPUT_LEN] ; let mut input_pos = 0 ; let mut output_pos = 0 ; while let Some ((mut code_point , new_pos)) = $ crate :: decode_utf8_char (INPUT , input_pos) { input_pos = new_pos ; if code_point <= 0xffff { buffer [output_pos] = code_point as u16 ; output_pos += 1 ; } else { code_point -= 0x10000 ; buffer [output_pos] = 0xd800 + (code_point >> 10) as u16 ; output_pos += 1 ; buffer [output_pos] = 0xdc00 + (code_point & 0x3ff) as u16 ; output_pos += 1 ; } } & { buffer } } ; $ crate :: PCWSTR :: from_raw (OUTPUT . as_ptr ()) } } ; }
};
}
