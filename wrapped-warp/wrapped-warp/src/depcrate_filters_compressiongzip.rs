// Generated macro for gzip (function)
macro_rules! Depcrate_filters_compressiongzip {
() => {
// Module: crate::filters::compression
// Provides: {"gzip"}
// Dependencies: {}
# [doc = " Create a wrapping filter that compresses the Body of a [`Response`](crate::reply::Response)"] # [doc = " using gzip, adding `content-encoding: gzip` to the Response's [`HeaderMap`](hyper::HeaderMap)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::get()"] # [doc = "     .and(warp::path::end())"] # [doc = "     .and(warp::fs::file(\"./README.md\"))"] # [doc = "     .with(warp::compression::gzip());"] # [doc = " ```"] # [cfg (feature = "compression-gzip")] pub fn gzip () -> Compression < impl Fn (CompressionProps) -> Response + Copy > { let func = move | mut props : CompressionProps | { let body = Body :: wrap_stream (ReaderStream :: new (GzipEncoder :: new (StreamReader :: new (props . body ,)))) ; props . head . headers . append (CONTENT_ENCODING , CompressionAlgo :: GZIP . into ()) ; props . head . headers . remove (CONTENT_LENGTH) ; Response :: from_parts (props . head , body) } ; Compression { func } }
};
}
