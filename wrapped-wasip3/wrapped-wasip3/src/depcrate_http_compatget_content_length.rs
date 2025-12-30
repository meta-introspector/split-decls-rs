// Generated macro for get_content_length (function)
macro_rules! Depcrate_http_compatget_content_length {
() => {
// Module: crate::http_compat
// Provides: {"get_content_length"}
// Dependencies: {}
fn get_content_length (headers : types :: Headers) -> Result < Option < u64 > , ErrorCode > { let values = headers . get (http :: header :: CONTENT_LENGTH . as_str ()) ; if values . len () > 1 { return Err (to_internal_error_code ("multiple content-length values")) ; } let Some (value_bytes) = values . into_iter () . next () else { return Ok (None) ; } ; let value_str = std :: str :: from_utf8 (& value_bytes) . map_err (to_internal_error_code) ? ; let value_i64 : i64 = value_str . parse () . map_err (to_internal_error_code) ? ; let value = value_i64 . try_into () . map_err (to_internal_error_code) ? ; Ok (Some (value)) }
};
}
