// Generated macro for impl_153 (impl)
macro_rules! Depcrate_contextimpl_153 {
() => {
// Module: crate::context
// Provides: {"impl_153"}
// Dependencies: {}
impl ValueNumber for Value { fn to_number (& self) -> Result < f64 , () > { match * self { Value :: Number (ref i) => Ok (i . as_f64 () . unwrap ()) , _ => Err (()) , } } }
};
}
