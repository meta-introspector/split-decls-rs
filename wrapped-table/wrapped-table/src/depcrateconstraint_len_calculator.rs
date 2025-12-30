// Generated macro for constraint_len_calculator (function)
macro_rules! Depcrateconstraint_len_calculator {
() => {
// Module: crate
// Provides: {"constraint_len_calculator"}
// Dependencies: {}
fn constraint_len_calculator (items : & [Data]) -> (u16 , u16 , u16) { let name_len = items . iter () . map (Data :: name) . map (UnicodeWidthStr :: width) . max () . unwrap_or (0) ; let address_len = items . iter () . map (Data :: address) . flat_map (str :: lines) . map (UnicodeWidthStr :: width) . max () . unwrap_or (0) ; let email_len = items . iter () . map (Data :: email) . map (UnicodeWidthStr :: width) . max () . unwrap_or (0) ; # [expect (clippy :: cast_possible_truncation)] (name_len as u16 , address_len as u16 , email_len as u16) }
};
}
