// Generated macro for validate_types (function)
macro_rules! Depcrate_inputvalidate_types {
() => {
// Module: crate::input
// Provides: {"validate_types"}
// Dependencies: {}
fn validate_types < 'de , D > (deserializer : D) -> Result < Vec < InputSetEntry > , D :: Error > where D : Deserializer < 'de > , { let v : Vec < InputSetEntry > = Vec :: deserialize (deserializer) ? ; let mut it = v . iter () ; if let Some (first) = it . next () { it . try_fold (first , | last , cur | { if last . 0 . len () == cur . 0 . len () { Ok (cur) } else { Err ("the length of the InputSets and the product lists must match" . to_string ()) } }) . map_err (de :: Error :: custom) ? ; } Ok (v) }
};
}
