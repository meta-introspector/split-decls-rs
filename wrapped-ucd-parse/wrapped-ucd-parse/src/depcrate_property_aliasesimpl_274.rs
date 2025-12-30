// Generated macro for impl_274 (impl)
macro_rules! Depcrate_property_aliasesimpl_274 {
() => {
// Module: crate::property_aliases
// Provides: {"impl_274"}
// Dependencies: {}
impl std :: str :: FromStr for PropertyAlias { type Err = Error ; fn from_str (line : & str) -> Result < PropertyAlias , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<abbrev>[^\s;]+)\s*;
                \s*(?P<long>[^\s;]+)\s*
                (?:;(?P<aliases>.*))?
                " ,) ; let re_aliases = regex ! (r"\s*(?P<alias>[^\s;]+)\s*;?\s*") ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid PropertyAliases line: '{}'" , line) , } ; let mut aliases = vec ! [] ; if let Some (m) = caps . name ("aliases") { for acaps in re_aliases . captures_iter (m . as_str ()) { let alias = acaps . name ("alias") . unwrap () . as_str () ; aliases . push (alias . to_string ()) ; } } Ok (PropertyAlias { abbreviation : caps . name ("abbrev") . unwrap () . as_str () . to_string () , long : caps . name ("long") . unwrap () . as_str () . to_string () , aliases , }) } }
};
}
