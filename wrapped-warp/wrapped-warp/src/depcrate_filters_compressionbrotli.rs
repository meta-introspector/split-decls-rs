// Generated macro for brotli (function)
macro_rules! Depcrate_filters_compressionbrotli {
() => {
// Module: crate::filters::compression
// Provides: {"brotli"}
// Dependencies: {}
# [doc = " Create a wrapping filter that compresses the Body of a [`Response`](crate::reply::Response)"] # [doc = " using brotli, adding `content-encoding: br` to the Response's [`HeaderMap`](hyper::HeaderMap)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::get()"] # [doc = "     .and(warp::path::end())"] # [doc = "     .and(warp::fs::file(\"./README.md\"))"] # [doc = "     .with(warp::compression::brotli());"] # [doc = " ```"] # [cfg (feature = "compression-brotli")] pub fn brotli () -> Compression < impl Fn (CompressionProps) -> Response + Copy > { let func = move | mut props : CompressionProps | { let body = Body :: wrap_stream (ReaderStream :: new (BrotliEncoder :: new (StreamReader :: new (props . body ,)))) ; props . head . headers . append (CONTENT_ENCODING , CompressionAlgo :: BR . into ()) ; props . head . headers . remove (CONTENT_LENGTH) ; Response :: from_parts (props . head , body) } ; Compression { func } }
};
}
