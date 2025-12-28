macro_rules! macro_403 {
    () => {
        fallback_impl ! { String , i64 , u64 , i32 , u32 , i16 , u16 , i8 , u8 , bool , f32 , f64 , usize , isize , PathBuf , }
    };
}

macro_403!();