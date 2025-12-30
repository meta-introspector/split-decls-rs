// Generated macro for impl_488 (impl)
macro_rules! Depcrate_filters_multipartimpl_488 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_488"}
// Dependencies: {}
impl FilterBase for FormOptions { type Extract = (FormData ,) ; type Error = Rejection ; type Future = FormFut ; fn filter (& self , _ : Internal) -> Self :: Future { let boundary = super :: header :: header2 :: < ContentType > () . and_then (| ct | { let mime = Mime :: from (ct) ; let mime = mime . get_param ("boundary") . map (| v | v . to_string ()) . ok_or_else (| | reject :: invalid_header ("content-type")) ; future :: ready (mime) }) ; let filt = boundary . and (super :: body :: body ()) . map (| boundary : String , body | { let body = BodyIoError (BodyDataStream :: new (body)) ; FormData { inner : FormDataInner :: new (body , & boundary) , } }) ; if let Some (max_length) = self . max_length { Box :: pin (super :: body :: content_length_limit (max_length) . and (filt) . filter (Internal) ,) } else { Box :: pin (filt . filter (Internal)) } } }
};
}
