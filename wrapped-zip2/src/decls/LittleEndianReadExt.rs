macro_rules! LittleEndianReadExt {
    () => {
        # [doc = " Helper methods for reading unsigned integers in little-endian form."] pub trait LittleEndianReadExt : Read { fn read_u16_le (& mut self) -> io :: Result < u16 > { let mut out = [0u8 ; 2] ; self . read_exact (& mut out) ? ; Ok (u16 :: from_le_bytes (out)) } fn read_u32_le (& mut self) -> io :: Result < u32 > { let mut out = [0u8 ; 4] ; self . read_exact (& mut out) ? ; Ok (u32 :: from_le_bytes (out)) } fn read_u64_le (& mut self) -> io :: Result < u64 > { let mut out = [0u8 ; 8] ; self . read_exact (& mut out) ? ; Ok (u64 :: from_le_bytes (out)) } }
    };
}

LittleEndianReadExt!();