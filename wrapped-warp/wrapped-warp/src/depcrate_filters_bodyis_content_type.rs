// Generated macro for is_content_type (function)
macro_rules! Depcrate_filters_bodyis_content_type {
() => {
// Module: crate::filters::body
// Provides: {"is_content_type"}
// Dependencies: {}
fn is_content_type < D : Decode > () -> impl Filter < Extract = () , Error = Rejection > + Copy { filter_fn (move | route | { let (type_ , subtype) = D :: MIME ; if let Some (value) = route . headers () . get (CONTENT_TYPE) { tracing :: trace ! ("is_content_type {}/{}? {:?}" , type_ , subtype , value) ; let ct = value . to_str () . ok () . and_then (| s | s . parse :: < mime :: Mime > () . ok ()) ; if let Some (ct) = ct { if ct . type_ () == type_ && ct . subtype () == subtype { future :: ok (()) } else { tracing :: debug ! ("content-type {:?} doesn't match {}/{}" , value , type_ , subtype) ; future :: err (reject :: unsupported_media_type ()) } } else { tracing :: debug ! ("content-type {:?} couldn't be parsed" , value) ; future :: err (reject :: unsupported_media_type ()) } } else if D :: WITH_NO_CONTENT_TYPE { tracing :: trace ! ("no content-type header, assuming {}/{}" , type_ , subtype) ; future :: ok (()) } else { tracing :: debug ! ("no content-type found") ; future :: err (reject :: unsupported_media_type ()) } }) }
};
}
