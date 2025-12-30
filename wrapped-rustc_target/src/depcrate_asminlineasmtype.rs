// Generated macro for InlineAsmType (enum)
macro_rules! Depcrate_asmInlineAsmType {
() => {
// Module: crate::asm
// Provides: {"InlineAsmType"}
// Dependencies: {}
# [doc = " Set of types which can be used with a particular register class."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum InlineAsmType { I8 , I16 , I32 , I64 , I128 , F16 , F32 , F64 , F128 , VecI8 (u64) , VecI16 (u64) , VecI32 (u64) , VecI64 (u64) , VecI128 (u64) , VecF16 (u64) , VecF32 (u64) , VecF64 (u64) , VecF128 (u64) , }
};
}
