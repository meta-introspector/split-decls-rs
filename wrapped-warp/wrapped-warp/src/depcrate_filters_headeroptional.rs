// Generated macro for optional (function)
macro_rules! Depcrate_filters_headeroptional {
() => {
// Module: crate::filters::header
// Provides: {"optional"}
// Dependencies: {}
# [doc = " Create a `Filter` that tries to parse the specified header, if it exists."] # [doc = ""] # [doc = " If the header does not exist, it yields `None`. Otherwise, it will try to"] # [doc = " parse as a `T`, and if it fails, a invalid header rejection is return. If"] # [doc = " successful, the filter yields `Some(T)`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " // Grab the `authorization` header if it exists."] # [doc = " let opt_auth = warp::header::optional::<String>(\"authorization\");"] # [doc = " ```"] pub fn optional < T > (name : & 'static str ,) -> impl Filter < Extract = One < Option < T > > , Error = Rejection > + Copy where T : FromStr + Send + 'static , { filter_fn_one (move | route | { tracing :: trace ! ("optional({:?})" , name) ; let result = route . headers () . get (name) . map (| value | { value . to_str () . map_err (| _ | reject :: invalid_header (name)) ? . parse :: < T > () . map_err (| _ | reject :: invalid_header (name)) }) ; match result { Some (Ok (t)) => future :: ok (Some (t)) , Some (Err (e)) => future :: err (e) , None => future :: ok (None) , } }) }
};
}
