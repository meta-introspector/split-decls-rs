// Generated macro for impl_101 (impl)
macro_rules! Depcrate_extracted_derived_numeric_valuesimpl_101 {
() => {
// Module: crate::extracted::derived_numeric_values
// Provides: {"impl_101"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedNumericValues { type Err = Error ; fn from_str (line : & str) -> Result < DerivedNumericValues , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<codepoints>[^\s;]+)\s*;
                \s*(?P<numeric_value_decimal>[^\s;]+)\s*;
                \s*;
                \s*(?P<numeric_value_fraction>[^\s;]+)\s*
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid PropList line: '{}'" , line) , } ; let codepoints = caps ["codepoints"] . parse () ? ; let numeric_value_decimal = caps ["numeric_value_decimal"] . to_string () ; let numeric_value_fraction = caps ["numeric_value_fraction"] . to_string () ; Ok (DerivedNumericValues { codepoints , numeric_value_decimal , numeric_value_fraction , }) } }
};
}
