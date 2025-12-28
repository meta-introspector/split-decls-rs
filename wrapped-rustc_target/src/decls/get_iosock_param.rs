macro_rules! get_iosock_param {
    () => {
        fn get_iosock_param (arch_lib_dir : & str) -> & 'static str { let target_dir = std :: env :: var ("QNX_TARGET") . unwrap_or_else (| _ | "QNX_TARGET_not_set_please_source_qnxsdp-env.sh" . into ()) ; let linker_param = format ! ("-L{target_dir}/{arch_lib_dir}/io-sock/lib") ; linker_param . leak () }
    };
}

get_iosock_param!();