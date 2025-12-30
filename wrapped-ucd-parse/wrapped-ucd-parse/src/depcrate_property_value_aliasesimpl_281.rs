// Generated macro for impl_281 (impl)
macro_rules! Depcrate_property_value_aliasesimpl_281 {
() => {
// Module: crate::property_value_aliases
// Provides: {"impl_281"}
// Dependencies: {}
impl std :: str :: FromStr for PropertyValueAlias { type Err = Error ; fn from_str (line : & str) -> Result < PropertyValueAlias , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<prop>[^\s;]+)\s*;
                \s*(?P<abbrev>[^\s;]+)\s*;
                \s*(?P<long>[^\s;]+)\s*
                (?:;(?P<aliases>.*))?
                " ,) ; let re_parts_ccc = regex ! (r"(?x)
                ^
                ccc;
                \s*(?P<num_class>[0-9]+)\s*;
                \s*(?P<abbrev>[^\s;]+)\s*;
                \s*(?P<long>[^\s;]+)
                " ,) ; let re_aliases = regex ! (r"\s*(?P<alias>[^\s;]+)\s*;?\s*") ; if line . starts_with ("ccc;") { let caps = match re_parts_ccc . captures (line . trim ()) { Some (caps) => caps , None => { return err ! ("invalid PropertyValueAliases (ccc) line") } } ; let n = match caps ["num_class"] . parse () { Ok (n) => n , Err (err) => { return err ! ("failed to parse ccc number '{}': {}" , & caps ["num_class"] , err) } } ; let abbrev = caps . name ("abbrev") . unwrap () . as_str () ; let long = caps . name ("long") . unwrap () . as_str () ; return Ok (PropertyValueAlias { property : line [0 .. 3] . to_string () , numeric : Some (n) , abbreviation : abbrev . to_string () , long : long . to_string () , aliases : vec ! [] , }) ; } let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid PropertyValueAliases line") , } ; let mut aliases = vec ! [] ; if let Some (m) = caps . name ("aliases") { for acaps in re_aliases . captures_iter (m . as_str ()) { let alias = acaps . name ("alias") . unwrap () . as_str () ; if alias == "#" { break ; } aliases . push (alias . to_string ()) ; } } Ok (PropertyValueAlias { property : caps . name ("prop") . unwrap () . as_str () . to_string () , numeric : None , abbreviation : caps . name ("abbrev") . unwrap () . as_str () . to_string () , long : caps . name ("long") . unwrap () . as_str () . to_string () , aliases , }) } }
};
}
