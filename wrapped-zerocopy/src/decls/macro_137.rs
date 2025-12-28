macro_rules! macro_137 {
    () => {
        define_type ! (A , "A 32-bit unsigned integer" , U32 , u32 , 32 , 4 , u32 :: from_be_bytes , u32 :: to_be_bytes , u32 :: from_le_bytes , u32 :: to_le_bytes , "unsigned integer" , [u64 , u128] , [u64 , u128] , [U64 , U128] , [U64 , U128]) ;
    };
}

macro_137!();