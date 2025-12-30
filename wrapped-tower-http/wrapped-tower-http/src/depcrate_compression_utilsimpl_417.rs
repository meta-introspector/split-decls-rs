// Generated macro for impl_417 (impl)
macro_rules! Depcrate_compression_utilsimpl_417 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_417"}
// Dependencies: {}
impl < B , M > Body for WrapBody < M > where B : Body , B :: Error : Into < BoxError > , M : DecorateAsyncRead < Input = AsyncReadBody < B > > , { type Data = Bytes ; type Error = BoxError ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < http_body :: Frame < Self :: Data > , Self :: Error > > > { let mut this = self . project () ; if ! * this . read_all_data { if this . buf . capacity () == 0 { this . buf . reserve (Self :: INTERNAL_BUF_CAPACITY) ; } let result = tokio_util :: io :: poll_read_buf (this . read . as_mut () , cx , & mut this . buf) ; match ready ! (result) { Ok (0) => { * this . read_all_data = true ; } Ok (_) => { let chunk = this . buf . split () . freeze () ; return Poll :: Ready (Some (Ok (Frame :: data (chunk)))) ; } Err (err) => { let body_error : Option < B :: Error > = M :: get_pin_mut (this . read) . get_pin_mut () . project () . error . take () ; if let Some (body_error) = body_error { return Poll :: Ready (Some (Err (body_error . into ()))) ; } else if err . raw_os_error () == Some (SENTINEL_ERROR_CODE) { unreachable ! () } else { return Poll :: Ready (Some (Err (err . into ()))) ; } } } } let body = M :: get_pin_mut (this . read) . get_pin_mut () . get_pin_mut () ; body . poll_frame (cx) . map (| option | { option . map (| result | { result . map (| frame | frame . map_data (| mut data | data . copy_to_bytes (data . remaining ()))) . map_err (| err | err . into ()) }) }) } }
};
}
