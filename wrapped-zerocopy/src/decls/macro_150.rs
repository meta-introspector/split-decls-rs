macro_rules! macro_150 {
    () => {
        define_type ! (An , "A 64-bit floating point number" , F64 , f64 , 64 , 8 , f64_ext :: from_be_bytes , f64_ext :: to_be_bytes , f64_ext :: from_le_bytes , f64_ext :: to_le_bytes , "floating point number" , [] , [] , [] , []) ;
    };
}

macro_150!()