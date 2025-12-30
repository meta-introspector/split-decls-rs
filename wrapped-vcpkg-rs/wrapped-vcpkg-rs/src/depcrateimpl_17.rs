// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl error :: Error for Error { fn description (& self) -> & str { match * self { Error :: DisabledByEnv (_) => "vcpkg-rs requested to be aborted" , Error :: RequiredEnvMissing (_) => "a required env setting is missing" , Error :: NotMSVC => "vcpkg-rs only can only find libraries for MSVC ABI builds" , Error :: VcpkgNotFound (_) => "could not find Vcpkg tree" , Error :: LibNotFound (_) => "could not find library in Vcpkg tree" , Error :: VcpkgInstallation (_) => "could not look up details of packages in vcpkg tree" , Error :: __Nonexhaustive => panic ! () , } } fn cause (& self) -> Option < & error :: Error > { match * self { _ => None , } } }
};
}
