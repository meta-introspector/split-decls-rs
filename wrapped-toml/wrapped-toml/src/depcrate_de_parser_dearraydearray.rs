// Generated macro for DeArray (struct)
macro_rules! Depcrate_de_parser_dearrayDeArray {
() => {
// Module: crate::de::parser::dearray
// Provides: {"DeArray"}
// Dependencies: {}
# [doc = " Type representing a TOML array, payload of the `DeValue::Array` variant"] # [derive (Clone)] pub struct DeArray < 'i > { items : Vec < Spanned < DeValue < 'i > > > , array_of_tables : bool , }
};
}
