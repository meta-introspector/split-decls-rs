// Generated macro for unversioned_llvm_target (function)
macro_rules! Depcrate_spec_base_appleunversioned_llvm_target {
() => {
// Module: crate::spec::base::apple
// Provides: {"unversioned_llvm_target"}
// Dependencies: {}
# [doc = " Generate part of the LLVM target triple."] # [doc = ""] # [doc = " See `rustc_codegen_ssa::back::versioned_llvm_target` for the full triple passed to LLVM and"] # [doc = " Clang."] fn unversioned_llvm_target (os : & str , arch : Arch , env : TargetEnv) -> StaticCow < str > { let arch = arch . target_name () ; let os = match os { "macos" => "macosx" , "ios" => "ios" , "watchos" => "watchos" , "tvos" => "tvos" , "visionos" => "xros" , _ => unreachable ! ("tried to get LLVM target OS for non-Apple platform") , } ; let environment = match env { TargetEnv :: Normal => "" , TargetEnv :: MacCatalyst => "-macabi" , TargetEnv :: Simulator => "-simulator" , } ; format ! ("{arch}-apple-{os}{environment}") . into () }
};
}
