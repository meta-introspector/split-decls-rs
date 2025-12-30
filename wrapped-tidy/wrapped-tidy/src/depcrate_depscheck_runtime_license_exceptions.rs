// Generated macro for check_runtime_license_exceptions (function)
macro_rules! Depcrate_depscheck_runtime_license_exceptions {
() => {
// Module: crate::deps
// Provides: {"check_runtime_license_exceptions"}
// Dependencies: {}
# [doc = " Check that all licenses of runtime dependencies are in the valid list in `LICENSES`."] # [doc = ""] # [doc = " Unlike for tools we don't allow exceptions to the `LICENSES` list for the runtime with the sole"] # [doc = " exception of `fortanix-sgx-abi` which is only used on x86_64-fortanix-unknown-sgx."] fn check_runtime_license_exceptions (metadata : & Metadata , bad : & mut bool) { for pkg in & metadata . packages { if pkg . source . is_none () { continue ; } let license = match & pkg . license { Some (license) => license , None => { tidy_error ! (bad , "dependency `{}` does not define a license expression" , pkg . id) ; continue ; } } ; if ! LICENSES . contains (& license . as_str ()) { if * pkg . name == "fortanix-sgx-abi" && pkg . license . as_deref () == Some ("MPL-2.0") { continue ; } tidy_error ! (bad , "invalid license `{}` in `{}`" , license , pkg . id) ; } } }
};
}
