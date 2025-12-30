// Generated macro for impl_169 (impl)
macro_rules! Depcrate_arabic_shapingimpl_169 {
() => {
// Module: crate::arabic_shaping
// Provides: {"impl_169"}
// Dependencies: {}
impl std :: str :: FromStr for ArabicShaping { type Err = Error ; fn from_str (line : & str) -> Result < ArabicShaping , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<codepoint>[A-F0-9]+)\s*;
                \s*(?P<name>[^;]+)\s*;
                \s*(?P<joining_type>[^;]+)\s*;
                \s*(?P<joining_group>[^;]+)
                $
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid ArabicShaping line") , } ; Ok (ArabicShaping { codepoint : caps ["codepoint"] . parse () ? , schematic_name : caps ["name"] . to_string () , joining_type : caps ["joining_type"] . parse () ? , joining_group : caps ["joining_group"] . to_string () , }) } }
};
}
