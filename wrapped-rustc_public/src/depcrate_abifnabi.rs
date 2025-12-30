// Generated macro for FnAbi (struct)
macro_rules! Depcrate_abiFnAbi {
() => {
// Module: crate::abi
// Provides: {"FnAbi"}
// Dependencies: {}
# [doc = " A function ABI definition."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct FnAbi { # [doc = " The types of each argument."] pub args : Vec < ArgAbi > , # [doc = " The expected return type."] pub ret : ArgAbi , # [doc = " The count of non-variadic arguments."] # [doc = ""] # [doc = " Should only be different from `args.len()` when a function is a C variadic function."] pub fixed_count : u32 , # [doc = " The ABI convention."] pub conv : CallConvention , # [doc = " Whether this is a variadic C function,"] pub c_variadic : bool , }
};
}
