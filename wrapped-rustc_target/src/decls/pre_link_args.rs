macro_rules! deps {
    () => {
        LinkArgs!();
        TargetOptions!();
        LinkerFlavor!();
        Lld!();
        Cc!();
        Arch!();
        ApiVariant!();
    };
}

macro_rules! pre_link_args {
    () => {
        deps!();
        pub (crate) fn pre_link_args (api_var : ApiVariant , arch : Arch) -> LinkArgs { let (qcc_arg , arch_lib_dir) = match arch { Arch :: Aarch64 => ("-Vgcc_ntoaarch64le_cxx" , "aarch64le") , Arch :: I586 => { ("-Vgcc_ntox86_cxx" , "notSupportedByQnx_compiler/rustc_target/src/spec/base/nto_qnx.rs") } Arch :: X86_64 => ("-Vgcc_ntox86_64_cxx" , "x86_64") , } ; match api_var { ApiVariant :: Default => { TargetOptions :: link_args (LinkerFlavor :: Gnu (Cc :: Yes , Lld :: No) , & [qcc_arg]) } ApiVariant :: IoSock => TargetOptions :: link_args (LinkerFlavor :: Gnu (Cc :: Yes , Lld :: No) , & [qcc_arg , get_iosock_param (arch_lib_dir)] ,) , } }
    };
}

pre_link_args!()