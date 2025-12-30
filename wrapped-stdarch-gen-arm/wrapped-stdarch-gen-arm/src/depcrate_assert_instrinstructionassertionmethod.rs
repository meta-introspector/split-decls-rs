// Generated macro for InstructionAssertionMethod (struct)
macro_rules! Depcrate_assert_instrInstructionAssertionMethod {
() => {
// Module: crate::assert_instr
// Provides: {"InstructionAssertionMethod"}
// Dependencies: {}
# [doc = " Asserts that the given instruction is present for the intrinsic of the associated type."] # [derive (Debug , Clone , Serialize , Deserialize)] # [serde (remote = "Self")] pub struct InstructionAssertionMethod { # [doc = " Instruction for integer intrinsics"] pub default : InstructionAssertionMethodForBitsize , # [doc = " Instruction for floating-point intrinsics (optional)"] # [serde (default)] pub float : Option < InstructionAssertionMethodForBitsize > , # [doc = " Instruction for unsigned integer intrinsics (optional)"] # [serde (default)] pub unsigned : Option < InstructionAssertionMethodForBitsize > , }
};
}
