// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl Display for Position { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (match self { Position :: Major => "major version number" , Position :: Minor => "minor version number" , Position :: Patch => "patch version number" , Position :: Pre => "pre-release identifier" , Position :: Build => "build metadata" , }) } }
};
}
