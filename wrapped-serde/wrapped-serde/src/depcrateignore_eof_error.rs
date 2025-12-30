// Generated macro for ignore_eof_error (function)
macro_rules! Depcrateignore_eof_error {
() => {
// Module: crate
// Provides: {"ignore_eof_error"}
// Dependencies: {}
pub fn ignore_eof_error < 'de , T , D > (result : Result < T , D >) -> Result < T , D > where T : Deserialize < 'de > + Default , D : std :: fmt :: Display , { match result { Err (err) if err . to_string () == "io error: unexpected end of file" => Ok (T :: default ()) , Err (err) if err . to_string () == "io error: failed to fill whole buffer" => Ok (T :: default ()) , result => result , } }
};
}
