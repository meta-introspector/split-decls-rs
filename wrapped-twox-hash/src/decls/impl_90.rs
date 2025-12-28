macro_rules! deps {
    () => {
        U8SliceExt!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl U8SliceExt for [u8] { # [inline] fn first_u32 (& self) -> Option < u32 > { self . first_chunk () . copied () . map (u32 :: from_le_bytes) } # [inline] fn last_u32 (& self) -> Option < u32 > { self . last_chunk () . copied () . map (u32 :: from_le_bytes) } # [inline] fn first_u64 (& self) -> Option < u64 > { self . first_chunk () . copied () . map (u64 :: from_le_bytes) } # [inline] fn last_u64 (& self) -> Option < u64 > { self . last_chunk () . copied () . map (u64 :: from_le_bytes) } }
    };
}

impl_90!();