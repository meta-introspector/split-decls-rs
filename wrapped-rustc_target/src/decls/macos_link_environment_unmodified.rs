macro_rules! macos_link_environment_unmodified {
    () => {
        # [test] fn macos_link_environment_unmodified () { let all_macos_targets = [aarch64_apple_darwin :: target () , i686_apple_darwin :: target () , x86_64_apple_darwin :: target () ,] ; for target in all_macos_targets { assert_eq ! (target . link_env_remove , crate :: spec :: cvs ! ["IPHONEOS_DEPLOYMENT_TARGET" , "TVOS_DEPLOYMENT_TARGET" , "XROS_DEPLOYMENT_TARGET"] ,) ; } }
    };
}

macos_link_environment_unmodified!();