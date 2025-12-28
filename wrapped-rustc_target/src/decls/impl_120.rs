macro_rules! deps {
    () => {
        InlineAsmType!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl InlineAsmType { pub fn is_integer (self) -> bool { matches ! (self , Self :: I8 | Self :: I16 | Self :: I32 | Self :: I64 | Self :: I128) } pub fn size (self) -> Size { Size :: from_bytes (match self { Self :: I8 => 1 , Self :: I16 => 2 , Self :: I32 => 4 , Self :: I64 => 8 , Self :: I128 => 16 , Self :: F16 => 2 , Self :: F32 => 4 , Self :: F64 => 8 , Self :: F128 => 16 , Self :: VecI8 (n) => n * 1 , Self :: VecI16 (n) => n * 2 , Self :: VecI32 (n) => n * 4 , Self :: VecI64 (n) => n * 8 , Self :: VecI128 (n) => n * 16 , Self :: VecF16 (n) => n * 2 , Self :: VecF32 (n) => n * 4 , Self :: VecF64 (n) => n * 8 , Self :: VecF128 (n) => n * 16 , }) } }
    };
}

impl_120!();