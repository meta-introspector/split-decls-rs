macro_rules! macro_142 {
    () => {
        define_type ! (An , "A 32-bit signed integer" , I32 , i32 , 32 , 4 , i32 :: from_be_bytes , i32 :: to_be_bytes , i32 :: from_le_bytes , i32 :: to_le_bytes , "signed integer" , [i64 , i128] , [i64 , i128] , [I64 , I128] , [I64 , I128]) ;
    };
}

macro_142!();