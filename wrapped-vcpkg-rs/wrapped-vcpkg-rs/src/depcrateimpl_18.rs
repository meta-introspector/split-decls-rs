// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match * self { Error :: DisabledByEnv (ref name) => write ! (f , "Aborted because {} is set" , name) , Error :: RequiredEnvMissing (ref name) => write ! (f , "Aborted because {} is not set" , name) , Error :: NotMSVC => write ! (f , "the vcpkg-rs Vcpkg build helper can only find libraries built for the MSVC ABI.") , Error :: VcpkgNotFound (ref detail) => write ! (f , "Could not find Vcpkg tree: {}" , detail) , Error :: LibNotFound (ref detail) => { write ! (f , "Could not find library in Vcpkg tree {}" , detail) } Error :: VcpkgInstallation (ref detail) => write ! (f , "Could not look up details of packages in vcpkg tree {}" , detail) , Error :: __Nonexhaustive => panic ! () , } } }
};
}
