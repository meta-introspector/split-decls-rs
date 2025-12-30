// Generated macro for dt_err (function)
macro_rules! Depcrate_ser_mapdt_err {
() => {
// Module: crate::ser::map
// Provides: {"dt_err"}
// Dependencies: {}
fn dt_err (err : toml_datetime :: ser :: SerializerError) -> Error { match err { toml_datetime :: ser :: SerializerError :: InvalidFormat (err) => Error :: custom (err) , _ => Error :: date_invalid () , } }
};
}
