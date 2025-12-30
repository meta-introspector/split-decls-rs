// Generated macro for deflate (function)
macro_rules! Depcrate_filters_compressiondeflate {
() => {
// Module: crate::filters::compression
// Provides: {"deflate"}
// Dependencies: {}
# [doc = " Create a wrapping filter that compresses the Body of a [`Response`](crate::reply::Response)"] # [doc = " using deflate, adding `content-encoding: deflate` to the Response's [`HeaderMap`](hyper::HeaderMap)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::get()"] # [doc = "     .and(warp::path::end())"] # [doc = "     .and(warp::fs::file(\"./README.md\"))"] # [doc = "     .with(warp::compression::deflate());"] # [doc = " ```"] # [cfg (feature = "compression-gzip")] pub fn deflate () -> Compression < impl Fn (CompressionProps) -> Response + Copy > { let func = move | mut props : CompressionProps | { let body = Body :: wrap_stream (ReaderStream :: new (DeflateEncoder :: new (StreamReader :: new (props . body ,)))) ; props . head . headers . append (CONTENT_ENCODING , CompressionAlgo :: DEFLATE . into ()) ; props . head . headers . remove (CONTENT_LENGTH) ; Response :: from_parts (props . head , body) } ; Compression { func } }
};
}
