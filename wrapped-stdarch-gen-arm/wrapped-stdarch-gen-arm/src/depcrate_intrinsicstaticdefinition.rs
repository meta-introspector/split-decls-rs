// Generated macro for StaticDefinition (enum)
macro_rules! Depcrate_intrinsicStaticDefinition {
() => {
// Module: crate::intrinsic
// Provides: {"StaticDefinition"}
// Dependencies: {}
# [doc = " Static definition part of the signature. It may evaluate to a constant"] # [doc = " expression with e.g. `const imm: u64`, or a generic `T: Into<u64>`."] # [derive (Debug , Clone , SerializeDisplay , DeserializeFromStr)] pub enum StaticDefinition { # [doc = " Constant expression"] Constant (Argument) , # [doc = " Generic type"] Generic (String) , }
};
}
