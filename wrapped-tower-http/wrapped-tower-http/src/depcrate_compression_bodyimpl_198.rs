// Generated macro for impl_198 (impl)
macro_rules! Depcrate_compression_bodyimpl_198 {
() => {
// Module: crate::compression::body
// Provides: {"impl_198"}
// Dependencies: {}
impl < B > Body for CompressionBody < B > where B : Body , B :: Error : Into < BoxError > , { type Data = Bytes ; type Error = BoxError ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < http_body :: Frame < Self :: Data > , Self :: Error > > > { match self . project () . inner . project () { # [cfg (feature = "compression-gzip")] BodyInnerProj :: Gzip { inner } => inner . poll_frame (cx) , # [cfg (feature = "compression-deflate")] BodyInnerProj :: Deflate { inner } => inner . poll_frame (cx) , # [cfg (feature = "compression-br")] BodyInnerProj :: Brotli { inner } => inner . poll_frame (cx) , # [cfg (feature = "compression-zstd")] BodyInnerProj :: Zstd { inner } => inner . poll_frame (cx) , BodyInnerProj :: Identity { inner } => match ready ! (inner . poll_frame (cx)) { Some (Ok (frame)) => { let frame = frame . map_data (| mut buf | buf . copy_to_bytes (buf . remaining ())) ; Poll :: Ready (Some (Ok (frame))) } Some (Err (err)) => Poll :: Ready (Some (Err (err . into ()))) , None => Poll :: Ready (None) , } , } } fn size_hint (& self) -> http_body :: SizeHint { if let BodyInner :: Identity { inner } = & self . inner { inner . size_hint () } else { http_body :: SizeHint :: new () } } fn is_end_stream (& self) -> bool { if let BodyInner :: Identity { inner } = & self . inner { inner . is_end_stream () } else { false } } }
};
}
