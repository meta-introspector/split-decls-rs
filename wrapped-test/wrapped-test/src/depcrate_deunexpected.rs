// Generated macro for unexpected (function)
macro_rules! Depcrate_deunexpected {
() => {
// Module: crate::de
// Provides: {"unexpected"}
// Dependencies: {}
fn unexpected (token : Token) -> Error { de :: Error :: custom (format ! ("deserialization did not expect this token: {}" , token ,)) }
};
}
