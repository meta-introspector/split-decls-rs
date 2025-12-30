// Generated macro for LLVMLink (struct)
macro_rules! Depcrate_intrinsicLLVMLink {
() => {
// Module: crate::intrinsic
// Provides: {"LLVMLink"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct LLVMLink { # [doc = " LLVM link function name without namespace and types,"] # [doc = " e.g. `st1` in `llvm.aarch64.sve.st1.nxv4i32`"] pub name : WildString , # [doc = " LLVM link signature arguments, leave unset if it inherits from intrinsic's signature"] pub arguments : Option < Vec < Argument > > , # [doc = " LLVM link signature return type, leave unset if it inherits from intrinsic's signature"] pub return_type : Option < TypeKind > , # [doc = " **This will be set automatically if not set**"] # [doc = " Attribute LLVM links for the function. First element is the architecture it targets,"] # [doc = " second element is the LLVM link itself."] pub links : Option < Vec < LLVMLinkAttribute > > , # [doc = " **Internal use only. Do not set.**"] # [doc = " Generated signature from these `arguments` and/or `return_type` if set, and the intrinsic's signature."] # [serde (skip)] pub signature : Option < Box < Signature > > , }
};
}
