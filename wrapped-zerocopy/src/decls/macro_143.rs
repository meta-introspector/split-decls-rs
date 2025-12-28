macro_rules! macro_143 {
    () => {
        define_type ! (An , "A 64-bit signed integer" , I64 , i64 , 64 , 8 , i64 :: from_be_bytes , i64 :: to_be_bytes , i64 :: from_le_bytes , i64 :: to_le_bytes , "signed integer" , [i128] , [i128] , [I128] , [I128]) ;
    };
}

macro_143!()