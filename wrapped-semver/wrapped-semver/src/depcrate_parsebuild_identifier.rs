// Generated macro for build_identifier (function)
macro_rules! Depcrate_parsebuild_identifier {
() => {
// Module: crate::parse
// Provides: {"build_identifier"}
// Dependencies: {}
fn build_identifier (input : & str) -> Result < (BuildMetadata , & str) , Error > { let (string , rest) = identifier (input , Position :: Build) ? ; let identifier = unsafe { Identifier :: new_unchecked (string) } ; Ok ((BuildMetadata { identifier } , rest)) }
};
}
