macro_rules! macro_149 {
    () => {
        define_type ! (An , "A 32-bit floating point number" , F32 , f32 , 32 , 4 , f32_ext :: from_be_bytes , f32_ext :: to_be_bytes , f32_ext :: from_le_bytes , f32_ext :: to_le_bytes , "floating point number" , [f64] , [] , [F64] , []) ;
    };
}

macro_149!();