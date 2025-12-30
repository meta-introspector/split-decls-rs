// Generated macro for impl_256 (impl)
macro_rules! Depcrate_name_aliasesimpl_256 {
() => {
// Module: crate::name_aliases
// Provides: {"impl_256"}
// Dependencies: {}
impl std :: str :: FromStr for NameAlias { type Err = Error ; fn from_str (line : & str) -> Result < NameAlias , Error > { let re_parts = regex ! (r"(?x)
                ^
                (?P<codepoint>[A-Z0-9]+);
                \s*
                (?P<alias>[^;]+);
                \s*
                (?P<label>\S+)
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid NameAliases line") , } ; Ok (NameAlias { codepoint : caps ["codepoint"] . parse () ? , alias : caps . name ("alias") . unwrap () . as_str () . to_string () , label : caps ["label"] . parse () ? , }) } }
};
}
