macro_rules! deps {
    () => {
        InlineAsmType!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl fmt :: Display for InlineAsmType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: I8 => f . write_str ("i8") , Self :: I16 => f . write_str ("i16") , Self :: I32 => f . write_str ("i32") , Self :: I64 => f . write_str ("i64") , Self :: I128 => f . write_str ("i128") , Self :: F16 => f . write_str ("f16") , Self :: F32 => f . write_str ("f32") , Self :: F64 => f . write_str ("f64") , Self :: F128 => f . write_str ("f128") , Self :: VecI8 (n) => write ! (f , "i8x{n}") , Self :: VecI16 (n) => write ! (f , "i16x{n}") , Self :: VecI32 (n) => write ! (f , "i32x{n}") , Self :: VecI64 (n) => write ! (f , "i64x{n}") , Self :: VecI128 (n) => write ! (f , "i128x{n}") , Self :: VecF16 (n) => write ! (f , "f16x{n}") , Self :: VecF32 (n) => write ! (f , "f32x{n}") , Self :: VecF64 (n) => write ! (f , "f64x{n}") , Self :: VecF128 (n) => write ! (f , "f128x{n}") , } } }
    };
}

impl_121!()