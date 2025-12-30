// Generated macro for impl_491 (impl)
macro_rules! Depcrate_filters_multipartimpl_491 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_491"}
// Dependencies: {}
impl Part { # [doc = " Get the name of this part."] pub fn name (& self) -> & str { self . part . name () . unwrap_or_else (| | self . part . file_name () . expect ("checked for name previously")) } # [doc = " Get the filename of this part, if present."] pub fn filename (& self) -> Option < & str > { self . part . file_name () } # [doc = " Get the content-type of this part, if present."] pub fn content_type (& self) -> Option < & str > { let content_type = self . part . content_type () ; content_type . map (| t | t . as_ref ()) } # [doc = " Asynchronously get some of the data for this `Part`."] pub async fn data (& mut self) -> Option < Result < impl Buf , crate :: Error > > { future :: poll_fn (| cx | self . poll_next (cx)) . await } # [doc = " Convert this `Part` into a `Stream` of `Buf`s."] pub fn stream (self) -> impl Stream < Item = Result < impl Buf , crate :: Error > > { PartStream (self) } fn poll_next (& mut self , cx : & mut Context < '_ >) -> Poll < Option < Result < Bytes , crate :: Error > > > { match Pin :: new (& mut self . part) . poll_next (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (Some (Ok (bytes))) => Poll :: Ready (Some (Ok (bytes))) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Err (err))) => Poll :: Ready (Some (Err (crate :: Error :: new (err)))) , } } }
};
}
