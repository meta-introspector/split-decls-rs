macro_rules! StackProbeType {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , serde_derive :: Deserialize , schemars :: JsonSchema)] # [serde (tag = "kind")] # [serde (rename_all = "kebab-case")] pub enum StackProbeType { # [doc = " Don't emit any stack probes."] None , # [doc = " It is harmless to use this option even on targets that do not have backend support for"] # [doc = " stack probes as the failure mode is the same as if no stack-probe option was specified in"] # [doc = " the first place."] Inline , # [doc = " Call `__rust_probestack` whenever stack needs to be probed."] Call , # [doc = " Use inline option for LLVM versions later than specified in `min_llvm_version_for_inline`"] # [doc = " and call `__rust_probestack` otherwise."] InlineOrCall { # [serde (rename = "min-llvm-version-for-inline")] min_llvm_version_for_inline : (u32 , u32 , u32) , } , }
    };
}

StackProbeType!();