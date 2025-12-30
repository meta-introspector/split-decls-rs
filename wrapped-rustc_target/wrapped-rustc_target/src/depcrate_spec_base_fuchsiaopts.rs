// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_fuchsiaopts {
() => {
// Module: crate::spec::base::fuchsia
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { let pre_link_args = TargetOptions :: link_args (LinkerFlavor :: Gnu (Cc :: No , Lld :: No) , & ["--build-id" , "--hash-style=gnu" , "-z" , "max-page-size=4096" , "-z" , "now" , "-z" , "start-stop-visibility=hidden" , "-z" , "rodynamic" , "-z" , "separate-loadable-segments" , "-z" , "rel" , "--pack-dyn-relocs=relr" ,] ,) ; TargetOptions { os : "fuchsia" . into () , linker_flavor : LinkerFlavor :: Gnu (Cc :: No , Lld :: Yes) , linker : Some ("rust-lld" . into ()) , dynamic_linking : true , families : cvs ! ["unix"] , pre_link_args , pre_link_objects : crt_objects :: new (& [(LinkOutputKind :: DynamicNoPicExe , & ["Scrt1.o"]) , (LinkOutputKind :: DynamicPicExe , & ["Scrt1.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["Scrt1.o"]) , (LinkOutputKind :: StaticPicExe , & ["Scrt1.o"]) ,]) , position_independent_executables : true , has_thread_local : true , frame_pointer : FramePointer :: NonLeaf , .. Default :: default () } }
};
}
