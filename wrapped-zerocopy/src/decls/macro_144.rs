macro_rules! macro_144 {
    () => {
        define_type ! (An , "A 128-bit signed integer" , I128 , i128 , 128 , 16 , i128 :: from_be_bytes , i128 :: to_be_bytes , i128 :: from_le_bytes , i128 :: to_le_bytes , "signed integer" , [] , [] , [] , []) ;
    };
}

macro_144!()