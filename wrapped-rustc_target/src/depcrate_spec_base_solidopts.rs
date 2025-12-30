// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_solidopts {
() => {
// Module: crate::spec::base::solid
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts (kernel : & str) -> TargetOptions { TargetOptions { os : format ! ("solid_{kernel}") . into () , vendor : "kmc" . into () , executables : false , frame_pointer : FramePointer :: NonLeaf , has_thread_local : true , .. Default :: default () } }
};
}
