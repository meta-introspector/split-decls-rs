// Generated macro for string_from_literal (function)
macro_rules! Depcratestring_from_literal {
() => {
// Module: crate
// Provides: {"string_from_literal"}
// Dependencies: {}
fn string_from_literal (literal : Literal) -> String { let string_literal = literal . to_string () ; if ! string_literal . starts_with ('\"') || ! string_literal . ends_with ('\"') { panic ! ("Expected a string literal, got '{}'" , string_literal) ; } string_literal [1 .. string_literal . len () - 1] . to_string () }
};
}
