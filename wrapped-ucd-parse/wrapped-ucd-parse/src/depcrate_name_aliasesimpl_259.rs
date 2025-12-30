// Generated macro for impl_259 (impl)
macro_rules! Depcrate_name_aliasesimpl_259 {
() => {
// Module: crate::name_aliases
// Provides: {"impl_259"}
// Dependencies: {}
impl std :: str :: FromStr for NameAliasLabel { type Err = Error ; fn from_str (s : & str) -> Result < NameAliasLabel , Error > { match s { "correction" => Ok (NameAliasLabel :: Correction) , "control" => Ok (NameAliasLabel :: Control) , "alternate" => Ok (NameAliasLabel :: Alternate) , "figment" => Ok (NameAliasLabel :: Figment) , "abbreviation" => Ok (NameAliasLabel :: Abbreviation) , unknown => err ! ("unknown name alias label: '{}'" , unknown) , } } }
};
}
