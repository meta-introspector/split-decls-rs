macro_rules! macro_138 {
    () => {
        define_type ! (A , "A 64-bit unsigned integer" , U64 , u64 , 64 , 8 , u64 :: from_be_bytes , u64 :: to_be_bytes , u64 :: from_le_bytes , u64 :: to_le_bytes , "unsigned integer" , [u128] , [u128] , [U128] , [U128]) ;
    };
}

macro_138!()