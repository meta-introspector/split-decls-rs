// Generated macro for check_repr_packed (function)
macro_rules! Depcratecheck_repr_packed {
() => {
// Module: crate
// Provides: {"check_repr_packed"}
// Dependencies: {}
fn check_repr_packed (attrs : & [syn :: Attribute]) -> bool { fn is_valid_repr_for_packed (ident : & syn :: Ident) -> bool { ident == "C" || ident == "Rust" } struct CheckIsPacked (bool) ; impl syn :: parse :: Parse for CheckIsPacked { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { let first_token = input . parse :: < syn :: Ident > () ? ; if ! input . peek (syn :: Token ! [,]) { let is_packed = first_token == "packed" ; return Ok (Self (is_packed)) ; } input . parse :: < syn :: Token ! [,] > () ? ; let second_token = input . parse :: < syn :: Ident > () ? ; if is_valid_repr_for_packed (& first_token) && second_token == "packed" { return Ok (Self (true)) ; } if first_token == "packed" && is_valid_repr_for_packed (& second_token) { return Ok (Self (true)) ; } Ok (Self (false)) } } attrs . iter () . any (| attr | { let syn :: Meta :: List (meta) = & attr . meta else { return false ; } ; let Some (ident) = meta . path . get_ident () else { return false ; } ; if ident != "repr" { return false ; } syn :: parse2 :: < CheckIsPacked > (meta . tokens . clone ()) . unwrap () . 0 }) }
};
}
