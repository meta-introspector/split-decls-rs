// Generated macro for try_parse (function)
macro_rules! Depcrate_releasetry_parse {
() => {
// Module: crate::release
// Provides: {"try_parse"}
// Dependencies: {}
fn try_parse (iter : Iter) -> Result < Release , () > { let major_minor = token :: parse_literal (iter) . map_err (drop) ? ; let string = major_minor . to_string () ; if ! string . starts_with ("1.") { return Err (()) ; } let minor : u16 = string [2 ..] . parse () . map_err (drop) ? ; let patch = if token :: parse_optional_punct (iter , '.') . is_some () { let int = token :: parse_literal (iter) . map_err (drop) ? ; Some (int . to_string () . parse () . map_err (drop) ?) } else { None } ; Ok (Release { minor , patch }) }
};
}
