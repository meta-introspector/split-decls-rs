// Generated macro for try_parse (function)
macro_rules! Depcrate_datetry_parse {
() => {
// Module: crate::date
// Provides: {"try_parse"}
// Dependencies: {}
fn try_parse (iter : Iter) -> Result < Date , () > { let year = token :: parse_literal (iter) . map_err (drop) ? ; token :: parse_punct (iter , '-') . map_err (drop) ? ; let month = token :: parse_literal (iter) . map_err (drop) ? ; token :: parse_punct (iter , '-') . map_err (drop) ? ; let day = token :: parse_literal (iter) . map_err (drop) ? ; let year = year . to_string () . parse :: < u64 > () . map_err (drop) ? ; let month = month . to_string () . parse :: < u64 > () . map_err (drop) ? ; let day = day . to_string () . parse :: < u64 > () . map_err (drop) ? ; if year >= 3000 || month > 12 || day > 31 { return Err (()) ; } Ok (Date { year : year as u16 , month : month as u8 , day : day as u8 , }) }
};
}
