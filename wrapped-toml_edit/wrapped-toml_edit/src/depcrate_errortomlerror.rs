// Generated macro for TomlError (struct)
macro_rules! Depcrate_errorTomlError {
() => {
// Module: crate::error
// Provides: {"TomlError"}
// Dependencies: {}
# [doc = " A TOML parse error"] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub struct TomlError { message : String , input : Option < std :: sync :: Arc < str > > , keys : Vec < String > , span : Option < std :: ops :: Range < usize > > , }
};
}
