// Generated macro for get_features (function)
macro_rules! Depcrateget_features {
() => {
// Module: crate
// Provides: {"get_features"}
// Dependencies: {}
fn get_features (attrs : & [Attribute] , mut features : Tokens) -> Tokens { for attr in attrs { if path_eq (& attr . path , & "cfg" . into ()) { attr . to_tokens (& mut features) ; } } features }
};
}
