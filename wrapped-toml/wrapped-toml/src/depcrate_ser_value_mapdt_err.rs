// Generated macro for dt_err (function)
macro_rules! Depcrate_ser_value_mapdt_err {
() => {
// Module: crate::ser::value::map
// Provides: {"dt_err"}
// Dependencies: {}
fn dt_err (err : toml_datetime :: ser :: SerializerError) -> Error { match err { toml_datetime :: ser :: SerializerError :: InvalidFormat (err) => Error :: new (err) , _ => Error :: date_invalid () , } }
};
}
