// Generated macro for http_from_wasi_response (function)
macro_rules! Depcrate_http_compat_conversionshttp_from_wasi_response {
() => {
// Module: crate::http_compat::conversions
// Provides: {"http_from_wasi_response"}
// Dependencies: {}
# [doc = " Converts a WASI HTTP response (`WasiHttpResponse`) into a standard host-side"] # [doc = " [`http::Response`] suitable for use with Rust’s `http` ecosystem."] # [doc = ""] # [doc = " This function performs the reverse operation of [`http_into_wasi_response`], translating"] # [doc = " the fields and body of a response from the WASI HTTP model into the conventional Rust"] # [doc = " `http` crate representation."] # [doc = " "] # [doc = " # See Also"] # [doc = ""] # [doc = " - [`http_into_wasi_response`] — the inverse conversion."] # [doc = " - [`IncomingResponseBody`] — for handling WASI-to-host body streams."] # [doc = " - [`ErrorCode`] — for standardized error reporting."] pub fn http_from_wasi_response (resp : WasiHttpResponse) -> Result < HttpResponse , ErrorCode > { let mut builder = http :: Response :: builder () . status (resp . get_status_code ()) ; for (k , v) in resp . get_headers () . copy_all () { builder = builder . header (k , v) ; } let body = IncomingResponseBody :: new (resp) ? ; builder . body (body) . map_err (to_internal_error_code) }
};
}
