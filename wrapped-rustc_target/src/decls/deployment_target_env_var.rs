macro_rules! deployment_target_env_var {
    () => {
        # [doc = " Name of the environment variable used to fetch the deployment target on the given OS."] pub fn deployment_target_env_var (os : & str) -> & 'static str { match os { "macos" => "MACOSX_DEPLOYMENT_TARGET" , "ios" => "IPHONEOS_DEPLOYMENT_TARGET" , "watchos" => "WATCHOS_DEPLOYMENT_TARGET" , "tvos" => "TVOS_DEPLOYMENT_TARGET" , "visionos" => "XROS_DEPLOYMENT_TARGET" , _ => unreachable ! ("tried to get deployment target env var for non-Apple platform") , } }
    };
}

deployment_target_env_var!();