macro_rules! macro_141 {
    () => {
        define_type ! (An , "A 16-bit signed integer" , I16 , i16 , 16 , 2 , i16 :: from_be_bytes , i16 :: to_be_bytes , i16 :: from_le_bytes , i16 :: to_le_bytes , "signed integer" , [i32 , i64 , i128 , isize] , [i32 , i64 , i128 , isize] , [I32 , I64 , I128 , Isize] , [I32 , I64 , I128 , Isize]) ;
    };
}

macro_141!();