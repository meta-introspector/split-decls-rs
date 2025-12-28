macro_rules! deps {
    () => {
        StaticCow!();
    };
}

macro_rules! link_env_remove {
    () => {
        deps!();
        fn link_env_remove (os : & 'static str) -> StaticCow < [StaticCow < str >] > { if os == "macos" { cvs ! ["IPHONEOS_DEPLOYMENT_TARGET" , "TVOS_DEPLOYMENT_TARGET" , "XROS_DEPLOYMENT_TARGET"] } else { cvs ! ["MACOSX_DEPLOYMENT_TARGET"] } }
    };
}

link_env_remove!()