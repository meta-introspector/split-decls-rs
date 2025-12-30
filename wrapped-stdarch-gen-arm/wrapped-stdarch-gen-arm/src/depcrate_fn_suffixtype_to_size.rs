// Generated macro for type_to_size (function)
macro_rules! Depcrate_fn_suffixtype_to_size {
() => {
// Module: crate::fn_suffix
// Provides: {"type_to_size"}
// Dependencies: {}
pub fn type_to_size (str_type : & str) -> i32 { match str_type { "int8x8_t" | "int8x16_t" | "i8" | "s8" | "uint8x8_t" | "uint8x16_t" | "u8" | "poly8x8_t" | "poly8x16_t" => 8 , "int16x4_t" | "int16x8_t" | "i16" | "s16" | "uint16x4_t" | "uint16x8_t" | "u16" | "float16x4_t" | "float16x8_t" | "_f16" | "poly16x4_t" | "poly16x8_t" => 16 , "int32x2_t" | "int32x4_t" | "i32" | "s32" | "uint32x2_t" | "uint32x4_t" | "u32" | "float32x2_t" | "float32x4_t" | "f32" => 32 , "int64x1_t" | "int64x2_t" | "i64" | "s64" | "uint64x1_t" | "uint64x2_t" | "u64" | "float64x1_t" | "float64x2_t" | "f64" | "poly64x1_t" | "poly64x2_t" | "p64" => 64 , "p128" => 128 , _ => panic ! ("unknown type: {str_type}") , } }
};
}
