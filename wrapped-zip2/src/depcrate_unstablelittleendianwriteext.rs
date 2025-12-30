// Generated macro for LittleEndianWriteExt (trait)
macro_rules! Depcrate_unstableLittleEndianWriteExt {
() => {
// Module: crate::unstable
// Provides: {"LittleEndianWriteExt"}
// Dependencies: {}
# [doc = " Helper methods for writing unsigned integers in little-endian form."] pub trait LittleEndianWriteExt : Write { fn write_u16_le (& mut self , input : u16) -> io :: Result < () > { self . write_all (& input . to_le_bytes ()) } fn write_u32_le (& mut self , input : u32) -> io :: Result < () > { self . write_all (& input . to_le_bytes ()) } fn write_u64_le (& mut self , input : u64) -> io :: Result < () > { self . write_all (& input . to_le_bytes ()) } fn write_u128_le (& mut self , input : u128) -> io :: Result < () > { self . write_all (& input . to_le_bytes ()) } }
};
}
