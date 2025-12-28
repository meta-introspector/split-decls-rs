macro_rules! macro_136 {
    () => {
        define_type ! (A , "A 16-bit unsigned integer" , U16 , u16 , 16 , 2 , u16 :: from_be_bytes , u16 :: to_be_bytes , u16 :: from_le_bytes , u16 :: to_le_bytes , "unsigned integer" , [u32 , u64 , u128 , usize] , [u32 , u64 , u128 , usize] , [U32 , U64 , U128 , Usize] , [U32 , U64 , U128 , Usize]) ;
    };
}

macro_136!()