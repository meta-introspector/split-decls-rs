// Generated macro for get_iosock_param (function)
macro_rules! Depcrate_spec_base_nto_qnxget_iosock_param {
() => {
// Module: crate::spec::base::nto_qnx
// Provides: {"get_iosock_param"}
// Dependencies: {}
fn get_iosock_param (arch_lib_dir : & str) -> & 'static str { let target_dir = std :: env :: var ("QNX_TARGET") . unwrap_or_else (| _ | "QNX_TARGET_not_set_please_source_qnxsdp-env.sh" . into ()) ; let linker_param = format ! ("-L{target_dir}/{arch_lib_dir}/io-sock/lib") ; linker_param . leak () }
};
}
