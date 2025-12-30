// Generated macro for Target (struct)
macro_rules! Depcrate_specTarget {
() => {
// Module: crate::spec
// Provides: {"Target"}
// Dependencies: {}
# [doc = " Everything `rustc` knows about how to compile for a specific target."] # [doc = ""] # [doc = " Every field here must be specified, and has no default value."] # [derive (PartialEq , Clone , Debug)] pub struct Target { # [doc = " Unversioned target tuple to pass to LLVM."] # [doc = ""] # [doc = " Target tuples can optionally contain an OS version (notably Apple targets), which rustc"] # [doc = " cannot know without querying the environment."] # [doc = ""] # [doc = " Use `rustc_codegen_ssa::back::versioned_llvm_target` if you need the full LLVM target."] pub llvm_target : StaticCow < str > , # [doc = " Metadata about a target, for example the description or tier."] # [doc = " Used for generating target documentation."] pub metadata : TargetMetadata , # [doc = " Number of bits in a pointer. Influences the `target_pointer_width` `cfg` variable."] pub pointer_width : u16 , # [doc = " Architecture to use for ABI considerations. Valid options include: \"x86\","] # [doc = " \"x86_64\", \"arm\", \"aarch64\", \"mips\", \"powerpc\", \"powerpc64\", and others."] pub arch : StaticCow < str > , # [doc = " [Data layout](https://llvm.org/docs/LangRef.html#data-layout) to pass to LLVM."] pub data_layout : StaticCow < str > , # [doc = " Optional settings with defaults."] pub options : TargetOptions , }
};
}
