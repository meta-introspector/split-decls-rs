// Generated macro for file_conditional (function)
macro_rules! Depcrate_filters_fsfile_conditional {
() => {
// Module: crate::filters::fs
// Provides: {"file_conditional"}
// Dependencies: {}
fn file_conditional (f : TkFile , path : ArcPath , conditionals : Conditionals ,) -> impl Future < Output = Result < File , Rejection > > + Send { file_metadata (f) . map_ok (move | (file , meta) | { let mut len = meta . len () ; let modified = meta . modified () . ok () . map (LastModified :: from) ; let resp = match conditionals . check (modified) { Cond :: NoBody (resp) => resp , Cond :: WithBody (range) => { bytes_range (range , len) . map (| (start , end) | { let sub_len = end - start ; let buf_size = optimal_buf_size (& meta) ; let stream = file_stream (file , buf_size , (start , end)) ; let body = Body :: wrap_stream (stream) ; let mut resp = Response :: new (body) ; if sub_len != len { * resp . status_mut () = StatusCode :: PARTIAL_CONTENT ; resp . headers_mut () . typed_insert (ContentRange :: bytes (start .. end , len) . expect ("valid ContentRange") ,) ; len = sub_len ; } let mime = mime_guess :: from_path (path . as_ref ()) . first_or_octet_stream () ; resp . headers_mut () . typed_insert (ContentLength (len)) ; resp . headers_mut () . typed_insert (ContentType :: from (mime)) ; resp . headers_mut () . typed_insert (AcceptRanges :: bytes ()) ; if let Some (last_modified) = modified { resp . headers_mut () . typed_insert (last_modified) ; } resp }) . unwrap_or_else (| BadRange | { let mut resp = Response :: new (Body :: empty ()) ; * resp . status_mut () = StatusCode :: RANGE_NOT_SATISFIABLE ; resp . headers_mut () . typed_insert (ContentRange :: unsatisfied_bytes (len)) ; resp }) } } ; File { resp , path } }) }
};
}
