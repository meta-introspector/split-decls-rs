// Generated macro for impl_111 (impl)
macro_rules! Depcrate_indicesimpl_111 {
() => {
// Module: crate::indices
// Provides: {"impl_111"}
// Dependencies: {}
impl Iterator for Utf16Indices < '_ > { type Item = (usize , u32) ; # [inline] fn next (& mut self) -> Option < (usize , u32) > { let (index , ch) = self . iter . get (self . front_offset) . map (| ch | { self . front_offset += 1 ; (self . front_offset - 1 , * ch) }) ? ; let mut ch = ch as u32 ; if (ch & 0xfc00) != 0xd800 { return Some ((index , ch)) ; } if let Some (next) = self . iter . get (self . front_offset) { let next = * next as u32 ; if (next & 0xfc00) == 0xdc00 { ch = ((ch & 0x3ff) << 10) + (next & 0x3ff) + 0x10000 ; self . front_offset += 1 ; } } Some ((index , ch)) } }
};
}
